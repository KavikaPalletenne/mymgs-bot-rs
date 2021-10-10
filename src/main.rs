// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

mod login;
mod timetable_service;
mod timetable;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() -> Result<()> {

    for i in 1..100000 {
        login::login().await?;
    }

    Ok(())
}
