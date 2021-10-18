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
use std::time::Instant;
use serde_json::value::Value::Null;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO: Pass in a master PgPool for each DB function instead of instantiating a new one each time (takes a lot of time)
pub async fn initialise_timetable(user_id: i64) -> Result<String> {
    let pool = establish_database_connection().await?;

    let synergetic_id = user::get_user_synergetic_id_by_user_id(user_id, &pool).await?;
    let local_date = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
    let fetched_date = chrono::NaiveDate::parse_from_str(&*local_date, "%Y-%m-%d").unwrap();

    let mut timetable_id = get_timetable_id_by_user_id_if_it_exists(user_id, &pool).await?; // Returns 0 if timetable doesn't exist

    if timetable_id == 0 {
        timetable_id = create_timetable(user_id, fetched_date, &pool).await?; // Create a new timetable
        let fetched = fetch_timetable_by_synergetic_id(synergetic_id, user_id, timetable_id, &pool).await?;

        return Ok("successful".to_string());
    }

    class::delete_all_classes_in_timetable(timetable_id, &pool).await?; // Delete existing classes
    let fetched = fetch_timetable_by_synergetic_id(synergetic_id, user_id, timetable_id, &pool).await?; // If timetable exists, get new set of classes and update fetched date
    let fetched = fetched.as_str();

    // Check if timetable exists on myMGS API
    match fetched {
        "successful" => println!("Timetable exists {}", synergetic_id),
        _ => { return Ok("unsuccessful".to_string()); }
    }

    update_timetable_by_user_id(user_id, fetched_date, &pool).await?;
    Ok("successful".to_string())
}

pub async fn fetch_timetable_by_synergetic_id(synergetic_id: i32, user_id: i64, timetable_id: i32, pool: &Pool<Postgres>) -> Result<String> {
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
    let mut d_number = 1;

    if json == Null {
        println!("No such timetable with synergetic id {}", synergetic_id);
        return Ok("unsuccessful".to_string());
    }

    let now = Instant::now();
    for i in 1..=7 {
        //    [day][period]
        // json[2][1]["ClassCodeDescription"]
        // json[3][1]["ClassCodeDescription"]
        // json[5][1]["ClassCodeDescription"]
        //     ...
        // json[2][2]["ClassCodeDescription"]
        // json[3][2]["ClassCodeDescription"]
        // json[5][2]["ClassCodeDescription"]
        let mut p_number = 1;
        for j in 2..=9 { // No. 2,3,5,6,8,9 for periods 1,2,3,4,5,6 (4 & 7 are Lunch and Recess)
            if j != 4 && j != 7 {
                let class_name = json[j][i]["ClassCodeDescription"].to_string();
                let teacher = json[j][i]["StaffName"].to_string();
                let day_number = d_number;
                let period_number = p_number;

                // Remove quotes
                let class_name = &class_name[1..(class_name.len()-1)].to_string();
                let teacher = &teacher[1..(teacher.len()-1)].to_string();
                //let now = Instant::now();
                class::create_class(
                    timetable_id, day_number, period_number,
                    class_name.clone(), teacher.clone(), pool
                ).await?;
                //println!("Pushed class [{}, {}] to database: {} μs", period_number, day_number, now.elapsed().as_micros());

                p_number += 1;
            }
        }
        d_number += 1;
    }

    println!("Pushed classes to database for user {}: {}ms", user_id, now.elapsed().as_millis());

    Ok("successful".to_string())
}



//////////////////
// CRUD Functions
/////////////////

// Create
pub async fn create_timetable(user_id: i64, fetched_date: NaiveDate, pool: &Pool<Postgres>) -> Result<i32> {
    let timetable_id = sqlx::query!(
        "INSERT INTO timetables (user_id, fetched_date)
         VALUES ($1, $2) RETURNING id",
        user_id, fetched_date
    ).fetch_one(pool).await?;

    Ok(timetable_id.id)
}

// Read
pub async fn get_timetable_by_user_id(user_id: i64, pool: &Pool<Postgres>) -> Result<Timetable> {
    let timetable = sqlx::query_as!(
        Timetable,
        "SELECT * FROM timetables WHERE user_id = $1",
        user_id
    ).fetch_one(pool).await?;

    Ok(timetable)
}

// Update
pub async fn update_timetable_by_user_id(user_id: i64, fetched_date: NaiveDate, pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query!(
        "UPDATE timetables SET fetched_date = $1 WHERE user_id = $2",
        fetched_date, user_id
    ).execute(pool).await?;

    Ok(())
}

// Delete - Also delete all classed associated with the timetable
pub async fn delete_timetable_by_user_id(user_id: i64, pool: &Pool<Postgres>) -> Result<()> {
    // Get timetable id to use in deleting classes
    let timetable_id = sqlx::query!(
        r#"SELECT id FROM timetables WHERE user_id = $1"#,
        user_id
    ).fetch_one(pool).await?.id;

    class::delete_all_classes_in_timetable(timetable_id, &pool).await?;

    sqlx::query!(
        "DELETE FROM timetables WHERE user_id = $1",
        user_id
    ).execute(pool).await?;

    Ok(())
}

// Check if timetable exists and return its id. If it doesn't exists, the returned id is 0
pub async fn get_timetable_id_by_user_id_if_it_exists(user_id: i64, pool: &Pool<Postgres>) -> Result<i32> {
    let timetable_id = sqlx::query!(
        "SELECT id FROM timetables
         WHERE EXISTS (SELECT id FROM timetables WHERE user_id = $1)",
        user_id
    ).fetch_all(pool).await?;

    if timetable_id.len() == 0 {
        return Ok(0);
    }

    Ok(timetable_id[0].id)
}