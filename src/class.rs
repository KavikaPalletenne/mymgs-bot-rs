use crate::persistence::establish_database_connection;
use crate::models::Class;
use sqlx::*;
use std::env;


//////////////////
// CRUD Functions
/////////////////

// Create
pub async fn create_class(
    timetable_id: i32, day_number: i16, period_number: i16,
    class_name: String, teacher: String, pool: &Pool<Postgres>
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO classes (timetable_id, day_number, period_number, name, teacher)
         VALUES ($1, $2, $3, $4, $5)",
        timetable_id, day_number, period_number, class_name, teacher
    ).execute(pool).await?;

    Ok(())
}

//Read
pub async fn get_all_classes_by_timetable_id(timetable_id: i32, pool: &Pool<Postgres>) -> Result<Vec<Class>> {
    let classes: Vec<Class> = sqlx::query_as!(
        Class,
        "SELECT * from classes WHERE timetable_id = $1 ORDER BY day_number, period_number ASC",
        timetable_id
    ).fetch_all(pool).await?;

    Ok(classes)
}

// Update
pub async fn update_class(
    timetable_id: i32, day_number: i16, period_number: i16,
    class_name: String, teacher: String, pool: &Pool<Postgres>
) -> Result<()> {
    sqlx::query!(
        "UPDATE classes SET timetable_id = $1, day_number = $2,
         period_number = $3, name = $4, teacher = $5",
        timetable_id, day_number, period_number, class_name, teacher
    ).execute(pool).await?;

    Ok(())
}

// Delete
pub async fn delete_all_classes_in_timetable(timetable_id: i32, pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query!(
        "DELETE FROM classes WHERE timetable_id = $1",
        timetable_id
    ).execute(pool).await?;

    Ok(())
}


/////////////////////
// Utility Functions
/////////////////////

pub async fn get_all_classes_for_day_by_timetable_id(timetable_id: i32, day_number: i16, pool: &Pool<Postgres>) -> Result<Vec<Class>> {
    let classes: Vec<Class> = sqlx::query_as!(
        Class,
        "SELECT * from classes WHERE timetable_id = $1 AND day_number = $2 ORDER BY period_number ASC",
        timetable_id, day_number
    ).fetch_all(pool).await?;

    Ok(classes)
}

// Using one instance of pool instead of instantiating a new one every time
pub async fn create_class_batch(
    timetable_id: i32, day_number: i16, period_number: i16,
    class_name: String, teacher: String, pool: &Pool<Postgres>
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO classes (timetable_id, day_number, period_number, name, teacher)
         VALUES ($1, $2, $3, $4, $5)",
        timetable_id, day_number, period_number, class_name, teacher
    ).execute(pool).await?;

    Ok(())
}

// Returns only class subject name TODO: Find out how to do this
// pub async fn get_all_classes_by_timetable_id_bare(timetable_id: i32) -> Result<Vec<Class>> {
//     let pool = establish_database_connection().await?;
//
//     let classes: Vec<Class> = sqlx::query_as!(
//         Class,
//         "SELECT (name) from classes WHERE timetable_id = $1 ORDER BY day_number, period_number ASC",
//         timetable_id
//     ).fetch_all(&pool).await?.unwrap();
//
//
//     Ok(classes)
// }