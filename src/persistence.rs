use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };
use std::env;
use dotenv::dotenv;

pub async fn establish_database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(database_url.as_str()).await?;

    Ok(pool)
}