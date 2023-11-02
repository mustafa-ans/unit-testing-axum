use axum::{Extension,Json};
use serde_json::{json,Value};
use sqlx::PgPool;

use crate::models;



pub async fn get_user_handler(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    let result = sqlx::query_as::<_, models::credentials::User>(
        "SELECT
                ID,
                Username,
                FirstName,
                LastName,
                Email,
                BirthDate,
                CreatedAt,
                UpdatedAt
            FROM 
                Users",
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "error".to_string()
    })?;

    
    Ok(Json(json!(result)))
}
