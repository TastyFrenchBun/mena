use thirtyfour::{prelude::*, support};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
	let mut caps = DesiredCapabilities::chrome();
	let driver = WebDriver::new("http://localhost:9515", caps).await?;

	println!("helo");

	driver.quit().await?;
	Ok(())
}
