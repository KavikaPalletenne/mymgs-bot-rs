use serde::{ Serialize, Deserialize };
use chrono::NaiveDate;

#[derive(Debug, Queryable, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64, // Same as the unique "snowflake" number (u64, but returned by Discord as a String). Stored as u64 to enable faster SQL queries) , not something like "Endveous#1689" so that users changing their username won't affect the bot.
    pub synergetic_user_id: i32,
    pub mgs_email: Option<String>, // TODO: Leave this in? Or remove as people will never trust this.
    pub mgs_password: Option<String>,
}

// #[derive(Insertable)]
// #[table_name="users"]
// pub struct NewUser<'a> {
//     pub id: &'a i64,
//     pub synergetic_user_id: &'a i32,
//     pub mgs_email: Option<&'a str>,
//     pub mgs_password: Option<&'a str>,
// }


#[derive(Debug, Queryable, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timetable {
    id: i32,
    user_id: i64,
    fetched_date: NaiveDate,
}

// #[derive(Insertable)]
// #[table_name="timetables"]
// pub struct NewTimetable<'a> {
//     pub id: &'a i32,
//     pub user_id: &'a i64,
//     pub fetched_date: &'a NaiveDate,
// }


// TODO: Connect to timetable with diesel::associations
#[derive(Debug, Queryable, Serialize, Deserialize, sqlx::FromRow)]
pub struct Class {
    id: i32,
    timetable_id: i32,
    day_number: i16, // Day 1 - 7
    period_number: i16, // The first period of the day would have value "1"

    name: String,
    teacher: String,
}

// #[derive(Insertable)]
// #[table_name="classes"]
// pub struct NewClass<'a> {
//     pub id: &'a i32,
//     pub timetable_id: &'a i32,
//     pub day_number: &'a i16, // Day 1 - 7
//     pub period_number: &'a i16, // The first period of the day would have value "1"
//
//     pub name: &'a str,
//     pub teacher: &'a str,
// }