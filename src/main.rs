// Requires TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID in ~/.config/drink-water/env.
// Silent from 20:00 to 09:00 in the system's local time zone.
use chrono::{Local, Timelike};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hour = Local::now().hour();
    if !(9..20).contains(&hour) {
        return Ok(());
    }
    dotenvy::from_path(format!("{}/.config/drink-water/env", env::var("HOME")?))?;
    reqwest::blocking::Client::new()
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            env::var("TELEGRAM_BOT_TOKEN")?
        ))
        .form(&[
            ("chat_id", env::var("TELEGRAM_CHAT_ID")?.as_str()),
            ("text", "💧 Drink some water — it's been an hour. Go drink a glass of water."),
        ])
        .send()?
        .error_for_status()?;
    Ok(())
}
