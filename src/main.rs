// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

mod auth;
mod timetable_service;
mod timetable;
mod student;

use std::time::Instant;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() -> Result<()> {

    let jh = tokio::task::spawn(run());
    jh.await?;

    Ok(())
}

// Multithreading // TODO: Add the Discord bot run code here, so it can be run on all threads.
async fn run() -> Result<()> {
    for i in 1..100000 {
        let now = Instant::now();

        auth::login().await?;

        let time_elapsed = now.elapsed();
        println!("Logged in using {:?}: {}ms", std::thread::current().id(), time_elapsed.as_millis());

    }

    Ok(())
}