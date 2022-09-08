use thirtyfour::{prelude::*};
use std::sync::mpsc::Receiver;

pub async fn spawn_driver(receiver: Receiver<()>) -> WebDriverResult<()> {
	println!("starting the driver");
	
	let mut caps = DesiredCapabilities::chrome();
	caps.set_binary("/usr/bin/chromium-browser")?;
	let driver = WebDriver::new("http://localhost:9515", caps).await?;
	
	driver.goto("https://www.google.com/search?q=eur+to+rub").await?;

	loop {
		// wait for a channel to send a message
		receiver.recv().expect("Received an error!");

		// refresh the page
		driver.refresh().await?;

		// get cost span
		let cost_span = driver.find(By::XPath(r#"//*[@id="knowledge-currency__updatable-data-column"]/div[1]/div[2]/span[1]"#)).await?;

		let cost = cost_span.text().await;

		match cost {
			Ok(cost) => {println!("{}", cost)},
			Err(err) => {println!("{}", err)}
		}
	}
}
