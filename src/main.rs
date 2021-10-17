// Discord bot that interfaces with myMGS to expose API data withing Discord
// Copyright (c) 2021 Kavika Palletenne

// Modules
pub mod auth; // MGS login service
pub mod persistence; // Has functions to enable concise fetching of users/timetables
pub mod models; // Holds data structs
pub mod user; // User service
pub mod timetable; // Timetable service
pub mod class; // Class Service

// Imports
use std::time::Instant; // Used for performance testing
use dotenv::dotenv;

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
    for i in 1..999999 {
        let now = Instant::now();
        //auth::login("kbpalletenne@student.mgs.vic.edu.au", "12062004").await?;

        let query = crate::user::create_user_by_id(i, 102760, "kbpalletenne", "password").await?;
        let user = crate::user::get_user_by_id(i).await?;
        let delete = crate::user::delete_user_by_id(i-1).await?;
        //let id = crate::timetable::initialise_timetable(436035620905943041).await?;
        let timetable = timetable::fetch_timetable_by_synergetic_id(102760).await?;
        let time_elapsed = now.elapsed();
        println!("Logged in using {:?}: {}ms", std::thread::current().id(), time_elapsed.as_millis());
        println!("Fetched DB User {:?}: {}ns", std::thread::current().id(), time_elapsed.as_nanos());

    }

    Ok(())
}