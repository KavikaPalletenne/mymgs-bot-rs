// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

// extern crates
extern crate dotenv;

// Modules
pub mod auth; // myMGS login service
//pub mod timetable; // Timetable service
pub mod user; // User service
//pub mod schema; // Auto-generated table macros
pub mod models; // Holds data structs
pub mod persistence; // Has functions to enable concise fetching of users/timetables

// Imports
use std::time::Instant; // Used for performance testing

// For code simplicity
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
pub async fn main() -> Result<()> {

    let jh = tokio::task::spawn(run());
    jh.await?;

    Ok(())
}

// Multithreading // TODO: Add the Discord bot run code here, so it can be run on all threads.
async fn run() -> Result<()> {
    loop {
        let now = Instant::now();

        auth::login().await?;

        let user = crate::user::get_user_by_id(123123).await?;
        let time_elapsed = now.elapsed();
        println!("Logged in using {:?}: {}ms", std::thread::current().id(), time_elapsed.as_millis());
        println!("Fetched DB User {:?}: {}ns", std::thread::current().id(), time_elapsed.as_nanos());

    }

    Ok(())
}