use std::sync::Arc;

use teloxide::{prelude::*, types::InputFile};
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

	match bot.send_photo(chat_id, InputFile::memory(screenshot)).await {
		Ok(_) => (),
		Err(err) => println!("{}", err)
	}

	return
}

