// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

// Modules
pub mod auth; // MGS login module
pub mod bot;
pub mod class; // Class CRUD & utility functions
pub mod day; // Day number module
pub mod models; // Holds data structs
pub mod persistence; // DB connection function
pub mod timetable; // Timetable CRUD & utility functions
pub mod user; // User CRUD & utility functions // Discord bot module

// Imports

// For code simplicity
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO: Find a way to start a postgres pool at startup and use that all the time (starting one for each Discord command takes time)
#[tokio::main]
pub async fn main() -> Result<()> {
    let jh = tokio::task::spawn(run());
    jh.await?
}

// Multithreading // TODO: Add the Discord bot run code here, so it can be run on all threads.
async fn run() -> Result<()> {
    // let now = Instant::now();
    //timetable::initialise_timetable(436035620905943041).await?;
    // let time_elapsed = now.elapsed();
    // println!("Fetched User Timetable {:?}: {}ms", std::thread::current().id(), time_elapsed.as_millis());

    bot::bot_run().await?;

    Ok(())
}
