use job_scheduler_ng::{JobScheduler, Job, Schedule};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod mena;

fn main() {
	let config = mena::config::get_config();
	let config = Arc::new(config);

	let mut bot = None;

	// check if user wants to use telegram
	if config.telegram.enabled {
		bot = Some(mena::telegram::start_bot(config.clone()))
	}

	// start webdriver asynchronously
	let runtime = tokio::runtime::Runtime::new().unwrap();
	runtime.spawn(mena::webdriver::spawn_driver(config.clone(), bot.clone()));

	// create scheduler
	let mut scheduler = JobScheduler::new();
	
	// get schedule from config
	let cron_schedule = config.schedule.clone();

	// parse schedule
	let schedule = match cron_schedule.parse::<Schedule>() {
		Ok(schedule) => schedule,
		Err(err) => panic!("[mena-rust] couldn't parse schedule! {}", err)
	};

	scheduler.add(Job::new(schedule, || {
		let driver_result = runtime.block_on(mena::webdriver::spawn_driver(config.clone(), bot.clone()));
		
		if let Err(err) = driver_result {
			println!("[mena-rust] {}", err.to_string())
		}
	}));

	// run it...
	loop {
		//scheduler.tick();
    	thread::sleep(Duration::from_secs(1));
	}
}
