use axum::{Extension,Json, extract::Query};
use chrono::NaiveDate;
use serde_json::{json,Value};
use sqlx::PgPool;
use crate::models::{self, delete_user::DeleteUser};



pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Query(update_user): Query<DeleteUser>,
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
        return Err("User not found. Cannot delete user.".to_string());
    }

    // Get the username of the user to be deleted
    let deleted_user = sqlx::query_scalar::<_, String>(
        "SELECT Username FROM Users WHERE Username = $1",
    )
    .bind(&update_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "Error getting username for deletion".to_string()
    })?;

    // Delete the user based on the provided username
    sqlx::query(
        "DELETE FROM Users WHERE Username = $1",
    )
    .bind(&update_user.username)
    .execute(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "Error deleting user".to_string()
    })?;

    Ok(Json(json!({"message": format!("User '{}' deleted successfully.", deleted_user)})))
}
