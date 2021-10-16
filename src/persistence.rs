use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;
use crate::models::*;

async fn establish_database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/*async fn get_user(id: u64) -> User {
    let connection = establish_database_connection();
    let results = "cyr";
}*/