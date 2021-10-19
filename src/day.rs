use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use crate::auth::login;
use crate::persistence::establish_database_connection;
use serde_json;
use hyper::{Client, Request, Body};
use hyper_tls::HttpsConnector;
use chrono::NaiveDate;
use std::time::Instant;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgQueryResult;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

//////////////////
// CRUD Functions
/////////////////

// Create
pub async fn create_day(date: NaiveDate, day_number: i16, pool: &Pool<Postgres>) -> Result<()> {
    let records = sqlx::query!(
        "SELECT day_number FROM days WHERE date = $1",
        date
    ).fetch_all(pool).await?; // Should not be returning any record

    if records.len() != 0 {
        return Ok(());
    }

    let result: PgQueryResult = sqlx::query!(
        "INSERT INTO days (date, day_number) VALUES ($1, $2)",
        date, day_number
    ).execute(pool).await?;
    Ok(())
}

// Read
pub async fn get_day_number_by_date(date: NaiveDate, pool: &Pool<Postgres>) -> Result<i16> {
    let records = sqlx::query!(
        "SELECT day_number FROM days WHERE date = $1",
        date
    ).fetch_all(pool).await?; // Should only be returning one record

    if records.len() == 0 {
        return Ok(0); // Return 0 if day number couldn't be found for given date
    }

    Ok(records[0].day_number)
}

// Update - not needed yet

// Delete
pub async fn delete_day_by_date(date: NaiveDate, pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query!(
        "DELETE FROM days WHERE date = $1",
        date
    ).execute(pool).await?;

    Ok(())
}


/////////////////////
// Utility Functions
/////////////////////

pub async fn get_day_numbers(iterations: i16, pool: &Pool<Postgres>) -> Result<()> {
    dotenv().ok();
    println!("Fetching day numbers");
    let username = env::var("MGS_USERNAME").expect("USERNAME must be set"); // Using master credentials
    let password = env::var("MGS_PASSWORD").expect("PASSWORD must be set");


    let timetable_api_url = format!("https://my.mgs.vic.edu.au/mg/api/timetable_week/102760/{}/S/student.json", chrono::offset::Local::now().format("%Y-%m-%d"));
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
    let now = Instant::now();
    let response = client.request(request).await?;
    println!("Got back timetable JSON: {}ms", now.elapsed().as_millis());
    let body_bytes = hyper::body::to_bytes(response).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
    let json: serde_json::Value = serde_json::from_str(&body.as_str()).expect("JSON was not formatted properly");

    // TODO: add multiple iterations of this
    // json[0][i]["Day"] & json[0][i]["Date"] contains the data where i is {1, 2, 3, 4, 5, 6, 7}
    for i in 1..=7 {
        let date_string = json[0][i]["Date"].to_string();

        // Remove quotation marks
        let date_string = &date_string[1..(date_string.len()-1)].to_string();

        let date = NaiveDate::parse_from_str(date_string, "%Y-%m-%d").unwrap();
        let day_number = i16::from_str(json[0][i]["Day"].to_string().as_str()).unwrap();
        create_day(date, day_number, pool).await?;
    }
    Ok(())
}