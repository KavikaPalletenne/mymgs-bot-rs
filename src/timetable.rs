use crate::persistence::establish_database_connection;
use crate::auth::login;
use crate::models::Timetable;
use crate::user;
use crate::class;
use sqlx::*;
use std::env;
use dotenv::dotenv;
use serde_json;
use hyper::{Client, Request, Body};
use hyper_tls::HttpsConnector;
use chrono::NaiveDate;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn initialise_timetable(user_id: i64) -> Result<()> {
    let synergetic_id = user::get_user_synergetic_id_by_user_id(user_id).await?;

    let timetable = fetch_timetable_by_synergetic_id(synergetic_id).await?;

    Ok(())
}

pub async fn fetch_timetable_by_synergetic_id(synergetic_id: i32) -> Result<()> {

    dotenv().ok();

    let username = env::var("MGS_USERNAME").expect("USERNAME must be set"); // Using master credentials
    let password = env::var("MGS_PASSWORD").expect("PASSWORD must be set");

    let timetable_api_url = format!("https://my.mgs.vic.edu.au/mg/api/timetable_week/{}/{}/S/student.json", synergetic_id, chrono::offset::Local::now().format("%Y-%m-%d"));
    let (simple_saml_session_id, simple_saml_auth_token_cookie, ssess_cookie) = login(username.as_str(), password.as_str()).await?;
    let cookie = format!("{}; {}; {}", simple_saml_session_id, simple_saml_auth_token_cookie, ssess_cookie);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method("GET")
        .uri(&timetable_api_url)
        .header("Cookie", &cookie)
        .body(Body::empty())
        .unwrap();

    let response = client.request(request).await?;
    let body_bytes = hyper::body::to_bytes(response).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
    let json: serde_json::Value = serde_json::from_str(&body.as_str()).expect("JSON was not formatted properly");

    println!("{}", json[2][1]["ClassCodeDescription"]); // Gets day 1, period 1
    //TODO: Get json out of response

    Ok(())
}



//////////////////
// CRUD Functions
/////////////////

// Create
pub async fn create_timetable(user_id: i64, fetched_date: NaiveDate) -> Result<()> {
    let pool = establish_database_connection().await?;

    sqlx::query!(
        "INSERT INTO timetables (user_id, fetched_date)
         VALUES ($1, $2)",
        user_id, fetched_date
    ).execute(&pool).await?;

    Ok(())
}

// Read
pub async fn get_timetable_by_user_id(user_id: i64) -> Result<Timetable> {
    let pool = establish_database_connection().await?;

    let timetable = sqlx::query_as!(
        Timetable,
        "SELECT * FROM timetables WHERE user_id = $1",
        user_id
    ).fetch_one(&pool).await?;

    Ok(timetable)
}

// Update
pub async fn update_timetable_by_user_id(user_id: i64, fetched_date: NaiveDate) -> Result<()> {
    let pool = establish_database_connection().await?;

    sqlx::query!(
        "UPDATE timetables SET fetched_date = $1 WHERE user_id = $2",
        fetched_date, user_id
    ).execute(&pool).await?;

    Ok(())
}

// Delete - Also delete all classed associated with the timetable
pub async fn delete_timetable_by_user_id(user_id: i64) -> Result<()> {
    let pool = establish_database_connection().await?;

    // Get timetable id to use in deleting classes
    let timetable_id = sqlx::query!(
        r#"SELECT id FROM timetables WHERE user_id = $1"#,
        user_id
    ).fetch_one(&pool).await?.id;

    class::delete_all_classes_in_timetable(timetable_id).await?;

    sqlx::query!(
        "DELETE FROM timetables WHERE user_id = $1",
        user_id
    ).execute(&pool).await?;

    Ok(())
}