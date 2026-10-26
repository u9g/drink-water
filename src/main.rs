// Requires TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID in ~/.config/drink-water/env.
// Silent from 20:00 to 09:00 in the system's local time zone.
use chrono::{Local, Timelike};
use std::{collections::HashMap, env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hour = Local::now().hour();
    if !(9..20).contains(&hour) {
        return Ok(());
    }
    let config = fs::read_to_string(format!("{}/.config/drink-water/env", env::var("HOME")?))?;
    let vars: HashMap<_, _> = config.lines().filter_map(|l| l.split_once('=')).collect();
    reqwest::blocking::Client::new()
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            vars["TELEGRAM_BOT_TOKEN"]
        ))
        .form(&[
            ("chat_id", vars["TELEGRAM_CHAT_ID"]),
            ("text", "💧 Drink some water — it's been an hour. Go drink a glass of water."),
        ])
        .send()?
        .error_for_status()?;
    Ok(())
}
