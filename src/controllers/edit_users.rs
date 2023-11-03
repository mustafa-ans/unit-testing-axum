use axum::{Extension,Json};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{json,Value};
use sqlx::PgPool;
use crate::models;

use super::add_user::NewUser;


pub async fn _update_user(
    Json(update_user): Json<NewUser>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    // Check if the user with the given username exists
    let user_exists = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM Users WHERE Username = $1",
    )
    .bind(&update_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "Error checking user existence".to_string()
    })?;

    if user_exists == 0 {
        return Err("User not found. Cannot update user details.".to_string());
    }

    // Update user details based on provided fields
    sqlx::query(
        r#"
        UPDATE Users
        SET
            First_Name = COALESCE($2, First_Name),
            Last_Name = COALESCE($3, Last_Name),
            Email = COALESCE($4, Email),
            BirthDate = COALESCE($5, BirthDate)
        WHERE Username = $1
        "#,
    )
    .bind(&update_user.username)
    .bind(&update_user.first_name)
    .bind(&update_user.last_name)
    .bind(&update_user.email)
    .bind(&update_user.birthdate)
    .execute(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "Error updating user details".to_string()
    })?;

    Ok(Json(json!({"message": "User details updated successfully."})))
}
