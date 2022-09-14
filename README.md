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

## Setting up
### Pterodactyl
It's simple, you just need to import the egg into Pterodactyl
1. In your panel go to the Nests section in the admin part of the panel
2. Click the green Import Egg button
3. Browse to the `egg-nginx.json` , then select what nest you want to put the egg in.

	*If you want a new nest you need to create it before importing the egg.*
4. Go to the *Servers*, click "Create New", and choose your nest in the "Nest Configuration" section

### Building
First, clone the repository
```bash
git clone https://github.com/TastyFrenchBun/mena-rust.git
```
If you want to use it with **Docker**, then just run this:
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