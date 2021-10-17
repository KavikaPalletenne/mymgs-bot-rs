extern crate diesel;
use diesel::prelude::*;

use crate::persistence::establish_database_connection;
use crate::models::User;
use sqlx::*;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn get_user_by_id(user_id: i64) -> Result<User> {

    let pool = establish_database_connection().await?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(&pool)
        .await?;

    println!("User id: {:?}", user.mgs_email);

    Ok(user)
}

// TODO: Use this tutorial to implement CRUD for structs: https://cetra3.github.io/blog/implementing-a-jobq-sqlx/