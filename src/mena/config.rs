use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
	pub first_currency: String,
	pub second_currency: String,
	pub bot_token: String,
}

fn create_config_file() -> std::io::Result<()> {
	let config = Config {
		first_currency: "EUR".to_string(),
		second_currency: "RUB".to_string(),
		bot_token: "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
	};

	let config_as_string = toml::to_string(&config).unwrap();

	return std::fs::write("./config.toml", config_as_string)
}

fn read_toml() -> Config {
	let file = std::fs::read_to_string("./config.toml");

	let contents = match file {
		Err(error) => {
			panic!("{}", error)
		},
		Ok(file) => file
	};

	match toml::from_str(&contents) {
		Err(error) => {
			println!("Config is incorrect: {}", error.to_string());
			std::process::exit(1)
		},
		Ok(deserialized) => deserialized
	}
}

pub fn get_config() -> Config {
	if !std::path::Path::new("./config.toml").exists() {
		println!("Config file doesn't exist\nCreating new one...");

		let result = create_config_file();
		match result {
			Err(error) => {
				panic!("{}", error)
			},
			Ok(_) => {}
		}
	}

	return read_toml()
}
