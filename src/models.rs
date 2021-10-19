use serde::{ Serialize, Deserialize };
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64, // Same as the unique "snowflake" number (u64, but returned by Discord as a String. Stored as i64 to enable faster SQL queries) , not something like "Endveous#1689" so that users changing their username won't affect the bot.
    pub synergetic_user_id: i32,
    pub mgs_email: Option<String>,
    pub mgs_password: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timetable {
    pub id: i32,
    pub user_id: i64,
    pub fetched_date: NaiveDate,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Class {
    pub id: i32,
    pub timetable_id: i32,
    pub day_number: i16, // Day 1-7
    pub period_number: i16, // The first period of the day would have value "1"

    pub name: String,
    pub teacher: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Day {
    pub date: NaiveDate,
    pub day_number: i16, // Day 1-7
}