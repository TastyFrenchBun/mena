![](https://img.shields.io/github/license/TastyFrenchBun/mena-rust)
![](https://img.shields.io/badge/-t.me%2Fmenacurrency-blueviolet?logo=telegram&link=https://t.me/menacurrency)
# mena-rust 

Tracker for currency written in Rust  

## Purpose

With **mena-rust** you can track any currency changes and optionally send it to **Telegram**  
It also was made as an example on how to put your Rust program inside a Pterodactyl

## Scheduling
**mena-rust** uses cron format for schedules.

For example, expression
`*/10 * * * * * *` will run each 10th second.  
You can check out cron expression generator [here](https://crontab.cronhub.io)

## Telegram
If you want to use **Telegram**, then follow these steps:  
1. Go to the [@BotFather](https://t.me/BotFather) and create a new bot
2. Click *"API Token"* button and copy value that should look like this — `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`
3. Go to the `config.toml`, enable telegram and paste your bot token

	Example:
	```toml
	[telegram]
	enabled = true
	bot_token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
	chat_id = "123456789"

	```
4. Restart *mena-rust*

If you want to use *mena-rust* with a channel, then you should be aware that bots have different IDs than users  

But you can still find your channel by supplying its username:
```toml
chat_id = "@menacurrency"
```

## Setting up
### Pterodactyl
It's simple, you just need to import the egg into Pterodactyl
1. Download [this egg](https://github.com/TastyFrenchBun/mena-rust/blob/main/docker/pterodactyl/egg-mena-rust.json)
2. In your panel go to the Nests section in the admin part of the panel
3. Click the green Import Egg button
4. Browse to the `egg-nginx.json` , then select what nest you want to put the egg in.

	*If you want a new nest you need to create it before importing the egg.*
5. Go to the *Servers*, click "Create New", and choose your nest in the "Nest Configuration" section

### Building
First, clone the repository
```bash
git clone https://github.com/TastyFrenchBun/mena-rust.git
```
If you want to use it **with Docker**, then just run this:
```bash
cd mena-rust

# Basic
docker compose build native

# For Pterodactyl
docker compose build pterodactyl
```
If you want to use it without Docker, then:
```bash
cd mena-rust
cargo build --release
```

# License

*mena-rust* is distributed under GPL-3.0