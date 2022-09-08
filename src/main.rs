use clokwerk::{Scheduler, TimeUnits};
use std::sync::mpsc::{channel};
use std::thread;
use std::time::Duration;

mod mena;

fn main() {
	//let config = mena::config::get_config();

	let (sender, receiver) = channel();

	// start webdriver asynchronously
	let runtime = tokio::runtime::Runtime::new().unwrap();
	runtime.spawn(mena::webdriver::spawn_driver(receiver));

	// create scheduler
	let mut scheduler = Scheduler::new();
	scheduler.every(1.seconds()).run(move || {
		match sender.send(()) {
			Ok(_) => {}, // add log
			Err(err) => println!("{}", err)
		}
	});

	// run it...
	loop {
		scheduler.run_pending();
    	thread::sleep(Duration::from_millis(100));
	}
}
