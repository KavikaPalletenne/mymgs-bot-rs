use crate::persistence::establish_database_connection;
use crate::auth::login;
use crate::models::Timetable;
use crate::user;
use sqlx::*;
use std::env;
use dotenv::dotenv;



pub async fn initialise_timetable(user_id: i64) -> Result<()> {

    let synergetic_id = user::get_user_synergetic_id_by_id(user_id).await?;

    let timetable = fetch_timetable_by_synergetic_id(synergetic_id);

    Ok(())
}

async fn fetch_timetable_by_synergetic_id(synergetic_id: i32) -> Result<()> {
    let username = env::var("USERNAME") // Using master credentials
        .expect("USERNAME must be set");
    let password = env::var("PASSWORD")
        .expect("PASSWORD must be set");

    let (simple_saml_session_id, simple_saml_auth_token_cookie, ssess_cookie) = login(username.as_str(), password.as_str()).await?;
    // TODO: Implement the JSON fetching and parsing
    Ok(())
}