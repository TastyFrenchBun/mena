use std::sync::Arc;
use teloxide::{prelude::*, types::InputFile};
use image::*;
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
			println!("[mena-rust] telegram chat error => {}", err);
			return
		}
	};

	let mut img = match image::load_from_memory(screenshot.as_bytes()) {
		Ok(img) => img,
		Err(err) => {
			println!("[mena-rust] couldn't decode image => {}", err);
			return
		}
	};

	let cropped_img = img.crop(165, 175, 850, 510);
	let img_bytes = cropped_img.as_bytes().to_vec();

	match bot.send_photo(chat_id, InputFile::memory(img_bytes)).await {
		Ok(_) => (),
		Err(err) => println!("{}", err)
	}

	return
}

