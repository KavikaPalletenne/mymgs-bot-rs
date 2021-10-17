use crate::persistence::establish_database_connection;
use crate::models::User;
use sqlx::*;
use std::env;



//////////////////
// CRUD Functions
/////////////////

// Create
pub async fn create_user_by_id(id: i64, synergetic_user_id: i32, mgs_email: &str, mgs_password: &str) -> Result<()> {
    let pool = establish_database_connection().await?;

    sqlx::query!(
        "INSERT INTO users (id, synergetic_user_id, mgs_email, mgs_password)
         VALUES ($1, $2, $3, $4)",
        id, synergetic_user_id, mgs_email, mgs_password
    ).execute(&pool).await?;

    Ok(())
}

// Read
pub async fn get_user_by_id(id: i64) -> Result<User> {
    let pool = establish_database_connection().await?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
            id
        ).fetch_one(&pool).await?;

    println!("User id: {:?}", user.mgs_email);

    Ok(user)
}

// Update
pub async fn update_user_by_id(id: i64, synergetic_user_id: i32, mgs_email: &str, mgs_password: &str) -> Result<()> {
    let pool = establish_database_connection().await?;

    sqlx::query!(
        "UPDATE users SET synergetic_user_id = $1, mgs_email = $2, mgs_password = $3 WHERE id = $4",
        synergetic_user_id, mgs_email, mgs_password, id
    ).execute(&pool).await?;

    Ok(())
}

// Delete
pub async fn delete_user_by_id(id: i64) -> Result<()> {
    let pool = establish_database_connection().await?;

    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    ).execute(&pool).await?;

    Ok(())
}


/////////////////////
// Utility Functions
/////////////////////

pub async fn get_user_synergetic_id_by_user_id(id: i64) -> Result<i32> {
    let pool = establish_database_connection().await?;

    let record = sqlx::query!(
        r#"SELECT synergetic_user_id FROM users WHERE id = $1"#,
            id
        ).fetch_one(&pool).await?;

    Ok(record.synergetic_user_id)
}
