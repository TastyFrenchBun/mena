use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
	pub first_currency: String,
	pub second_currency: String,
	pub schedule: String,
	pub chromium_path: String,
	pub telegram: Telegram,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Telegram {
	pub enabled: bool,
	pub bot_token: String,
	pub chat_id: String,
}

fn create_config_file() -> std::io::Result<()> {
	let config = Config {
		first_currency: "EUR".to_string(),
		second_currency: "RUB".to_string(),
		schedule: "*/10 * * * * * *".to_string(),
		chromium_path: "/usr/bin/chromium".to_string(),
		telegram: Telegram { 
			enabled: false, 
			bot_token: "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
			chat_id: 123456789.to_string(),
		}
	};

	let config_as_string = toml::to_string(&config).unwrap();

	return std::fs::write("./config.toml", config_as_string)
}

fn get_path() -> String {
	// define default config path
	let mut path = String::from("./config.toml");

	// get custom path from ENV, if it's supplied
	if let Ok(env_path) = std::env::var("MENA_CONFIG") {
		path = env_path;
	}

	return path
}

fn read_toml() -> Config {
	// try to read the file to string
	let file = std::fs::read_to_string(get_path());

	// if it's good, assing it to `contents`
	let contents = match file {
		Err(error) => {
			panic!("{}", error)
		},
		Ok(file) => file
	};

	// then, try to parse it
	match toml::from_str(&contents) {
		Err(error) => {
			println!("Config is incorrect: {}", error.to_string());
			std::process::exit(1)
		},
		Ok(deserialized) => deserialized
	}
}

pub fn get_config() -> Config {
	// get path string
	let path = get_path();

	// wrap string as a path slice
	let config_file = std::path::Path::new(path.as_str());

	// check if it exists
	if !config_file.exists() {
		println!("Config file doesn't exist\nCreating new one...");

		let result = create_config_file();

		// panic if it didn't get created
		if let Err(err) = result {
			panic!("{}", err)
		}
	}

	return read_toml()
}
