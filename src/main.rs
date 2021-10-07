// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

mod login;
mod timetable_service;

use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() -> Result<()> {

    for i in 1..2 {
        let now = Instant::now();
        login::login().await?;

        println!("Logged in: {}ms", now.elapsed().as_millis());
    }

    Ok(())
}
