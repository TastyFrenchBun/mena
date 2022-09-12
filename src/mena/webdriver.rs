use std::sync::Arc;

use teloxide::{prelude::AutoSend, Bot};
use thirtyfour::prelude::{DesiredCapabilities, WebDriver, WebDriverResult, By, WebDriverError};
use crate::mena;

pub async fn spawn_driver(config: Arc<mena::config::Config>, bot: Option<AutoSend<Bot>>) -> WebDriverResult<()> {
	let mut caps = DesiredCapabilities::chrome();

	caps.set_no_sandbox()?;
	//caps.add_chrome_arg("--window-size=1920,1080")?;
	caps.add_chrome_arg("--lang=en-UK")?;
	caps.set_disable_gpu()?;
	caps.set_headless()?;
	caps.set_binary(config.chromium_path.as_str())?;

	// start the web browser
	let driver_result = WebDriver::new("http://localhost:9515", caps).await;

	let driver = match driver_result {
		Ok(driver) => driver,
		Err(err) => return Err(err)
	};

	driver.set_window_rect(0, 0, 1920, 1080).await?;
	
	let url = format!("https://www.google.com/search?q={}+to+{}", config.first_currency, config.second_currency);

	if let Err(err) = driver.goto(url).await {
		return Err(err);
	}

	// refresh the page
	driver.refresh().await?;

	// get cost span
	let find_result = driver.find(By::XPath(r#"//*[@id="knowledge-currency__updatable-data-column"]/div[1]/div[2]/span[1]"#)).await;

	let cost_span = match find_result {
		Ok(cost_span) => cost_span,
		Err(err) => {
			let mut url = String::from("unknown");

			if let Ok(current_url) = driver.current_url().await {
				url = current_url.to_string()
			}
			
			let error_msg = format!("couldn't find currency price span on page {}, error: {}", url, err.to_string());
			return Err(WebDriverError::CustomError(error_msg));
		}
	};

	match cost_span.text().await {
		Ok(cost) => println!("{} -> {} = {}", config.first_currency, config.second_currency, cost),
		Err(err) => println!("{}", err)
	}

	if let Some(bot) = bot {
		if let Ok(screenshot) = driver.screenshot_as_png().await {
			mena::telegram::post_currency(bot, config.telegram.chat_id.clone(), screenshot).await
		}
	}

	//drop(config);
	//let count = Arc::strong_count(&config);
	//println!("strong count {}", count);

	driver.quit().await?;
	Ok(())
}
