use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
// TODO: Change this into starting a connection instead of a pool as I think the connection might be quicker
pub async fn establish_database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(database_url.as_str())
        .await?;

    Ok(pool)
}
