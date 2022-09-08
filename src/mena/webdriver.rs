use thirtyfour::{prelude::*};
use std::sync::mpsc::Receiver;

pub async fn spawn_driver(receiver: Receiver<()>) {
	println!("[mena-rust] Starting the driver");
	
	let mut caps = DesiredCapabilities::chrome();

	// disable sandbox
	if let Err(err) = caps.set_no_sandbox() {
		println!("[mena-rust] couldn't disable sandbox! {}", err);
		return
	}

	// set window size
	if let Err(err) = caps.add_chrome_arg("--window-size=1920,1080") {
		println!("[mena-rust] couldn't set window size! {}", err);
		return
	}

	// disable gpu
	if let Err(err) = caps.set_disable_gpu() {
		println!("[mena-rust] couldn't disable gpu! {}", err);
		return
	}

	// set it to be headless
	if let Err(err) = caps.set_headless() {
		println!("[mena-rust] couldn't set chromium to headless! {}", err);
		return
	}

	// set chromium binary path
	if let Err(err) = caps.set_binary("/usr/bin/chromium") {
		println!("[mena-rust] couldn't set chromium binary path! {}", err);
		return
	}

	// get the driver
	let driver_result = WebDriver::new("http://localhost:9515", caps).await;

	let driver = match driver_result {
		Ok(driver) => driver,
		Err(err) => panic!("{}", err)
	};
	
	if let Err(err) = driver.goto("https://www.google.com/search?q=eur+to+rub").await {
		println!("[mena-rust] couldn't open the requested page! {}", err);
		return
	}

	loop {
		// wait for a channel to send a message
		receiver.recv().expect("Received an error!");

		// refresh the page
		if let Err(err) = driver.refresh().await {
			println!("[mena-rust] error while refreshing the page! {}", err);
			return
		}

		// get cost span
		let find_result = driver.find(By::XPath(r#"//*[@id="knowledge-currency__updatable-data-column"]/div[1]/div[2]/span[1]"#)).await;

		let cost_span = match find_result {
			Ok(cost_span) => cost_span,
			Err(err) => {
				if let Ok(url) = driver.current_url().await {
					println!("couldn't find cost span on page \"{}\", error: {}", url, err);
				} else {
					println!("a")
				}

				return;
			}
		};

		let cost = cost_span.text().await;

		match cost {
			Ok(cost) => {println!("{}", cost)},
			Err(err) => {println!("{}", err)}
		}
	}
}
