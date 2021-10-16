use crate::schema::*;

use serde::Serialize;
use chrono::NaiveDate;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i64, // Same as the unique "snowflake" number (u64, but returned by Discord as a String). Stored as u64 to enable faster SQL queries) , not something like "Endveous#1689" so that users changing their username won't affect the bot.
    synergetic_user_id: i32,
    mgs_email: Option<String>, // TODO: Leave this in? Or remove as people will never trust this.
    mgs_password: Option<String>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: &'a i64,
    pub synergetic_user_id: &'a i32,
    pub mgs_email: Option<&'a str>,
    pub mgs_password: Option<&'a str>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Timetable {
    id: i32,
    user_id: i64,
    fetched_date: NaiveDate,
}


// TODO: Connect iwth Diesel Associations
#[derive(Debug, Queryable, Serialize)]
pub struct Day {
    id: i32,
    timetable_id: i32,
    day_number: i16, // Day 1 would have the value "1"

}

#[derive(Debug, Queryable, Serialize)]
pub struct Class {
    id: i32,
    day_id: i32,
    period_number: i16, // The first period of the day would have value "1"

    name: String,
    teacher: String,
}