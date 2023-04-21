use axum::{Extension,Json};
use serde_json::{json,Value};
use sqlx::{PgPool};

use crate::{
    models,
};

pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String>{
    let user = sqlx::query_as::<_,models::auth::User>(
        "SELECT first_name, last_name FROM persons where first_name=$1",
    )
    .bind(&credentials.first_name)
    .fetch_optional(&pool)
    .await
    .map_err(|err|{
        dbg!(err);
        "error".to_string()
    })?;
    dbg!(&user);
    
    let result = sqlx::query("INSERT INTO persons (first_name, last_name) values($1, $2)")
        .bind(&credentials.first_name)
        .bind(&credentials.last_name)
        .execute(&pool)
        .await
        .map_err(|_| "error".to_string())?;
    if result.rows_affected()<1{
        Err("error".to_string())

    } else{
        Ok(Json(json!({"msg":"done"})))
    }
}



pub async fn get_user(
    Json(credentials): Json<models::auth::First>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    let result = sqlx::query_as::<_, models::auth::User>(
        "SELECT first_name, last_name FROM persons WHERE first_name=$1",
    )
    .bind(&credentials.first_name)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        "error".to_string()
    })?;
    dbg!(&result);
    if let Some(user) = result {
        let response = json!({
            "first_name": user.first_name,
            "last_name": user.last_name,
        });
        Ok(Json(response))
    } else {
        Err("User not found".to_string())
    }
}
