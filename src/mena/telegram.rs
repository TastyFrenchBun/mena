use std::sync::Arc;
use teloxide::{prelude::*, types::InputFile};
use image::{imageops, EncodableLayout};
use std::io::Cursor;
use crate::mena::config;

pub fn start_bot(config: Arc<config::Config>) -> AutoSend<Bot> {
	let bot = Bot::new(config.telegram.bot_token.clone()).auto_send();
	
	return bot;
}

pub async fn post_currency(bot: AutoSend<Bot>, chat_id: String, screenshot: Vec<u8>) {
	let chat_result = bot.get_chat(chat_id).await;

	let chat_id = match chat_result {
		Ok(chat) => chat.id,
		Err(err) => {
			println!("[mena-rust] telegram chat error -> {}", err);
			return
		}
	};

	let img = match image::load_from_memory(screenshot.as_bytes()) {
		Ok(img) => img,
		Err(err) => {
			println!("[mena-rust] couldn't decode image -> {}", err);
			return
		}
	};

	// sometimes even with --1920,1080 and same window size it still sets to 3840, 2160
	let mut resized_img = img.resize(1920, 1080, imageops::FilterType::CatmullRom);

	let cropped_img = resized_img.crop(165, 175, 850, 510);

	let mut cursor = Cursor::new(Vec::new());

	match cropped_img.write_to(&mut cursor, image::ImageFormat::Png) {
		Ok(_) => {
			//println!("[mena-rust] written image successfully! {}", cursor.get_ref().len())
		},
		Err(err) => {
			println!("[mena-rust] couldn't write image to Cursor -> {}", err);
			return
		}
	};

	let png_image = cursor.get_ref().to_owned();

	match bot.send_photo(chat_id, InputFile::memory(png_image)).await {
		Ok(_) => (),
		Err(err) => println!("[mena-rust] send_photo error -> {}", err)
	}

	return
}

