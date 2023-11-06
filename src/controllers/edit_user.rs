use axum::{Extension,Json};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{json,Value};
use sqlx::PgPool;
use crate::models::{self, edit_user::UpdateUser};



pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Json(update_user): Json<UpdateUser>,
) -> Result<Json<Value>, String> {
    // Check if the user with the given username exists
    let user_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM Users WHERE Username = $1",
    )
    .bind(&update_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        let err_message = format!("Error checking if user exists, error returned from the database: \n{}", err);
        println!("{}",&err_message);
        err_message
    })?;

    if user_exists == 0 {
        return Err("User not found. Cannot update user details.".to_string());
    }

    // Update user details based on provided fields
    sqlx::query(
        r#"
        UPDATE Users
        SET
            firstName = COALESCE($2, firstName),
            lastName = COALESCE($3, lastName),
            email = COALESCE($4, email),
            birthdate = COALESCE($5, birthdate)
        WHERE username = $1
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


#[cfg(test)]
mod tests {
    use axum::{http::{Request, Method}, body::Body, routing::put , Router};
    use sqlx_core::postgres::PgPoolOptions;
    use tower::util::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn check_database_connectivity(){
        let durl = std::env::var("DATABASE_URL_ONLIN").expect("set DATABASE_URL env variable");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&durl)
            .await
            .expect("unable to make connection");

        // assert to check if connection is successful
        assert_eq!(pool.is_closed(),false);

    }

    async fn create_connection_pool() -> PgPool{
        let durl = std::env::var("DATABASE_URL_ONLINE").expect("set DATABASE_URL env variable");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&durl)
            .await
            .expect("unable to make connection");

        return pool;
    }
    #[tokio::test]
    async fn test_update_user_route(){
        let pool = create_connection_pool().await;
        let app = Router::new()
            .route("/update-users",put(update_user))
            .layer(Extension(pool));

        let req = Request::builder()
            .method(Method::PUT)
            .uri("/update-users")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "username": "Test",
                    "first_name": "Test",
                    "last_name": "User",
                    "email": "test@mail",
                    "birthdate": "2020-01-01"
                }"#,
            ))
            .unwrap();

        let resp = app
            .oneshot(req)
            .await
            .unwrap();

        // extract the response body as bytes
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();

        // assert to check if user details are updated successfully 
        assert_eq!(body, r#"{"message":"User details updated successfully."}"#);
                
    }

    #[tokio::test]
    async fn test_update_user_not_found(){
        let pool = create_connection_pool().await;
        let app = Router::new()
            .route("/update-users",put(update_user))
            .layer(Extension(pool));

        let req = Request::builder()
            .method(Method::PUT)
            .uri("/update-users")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "username": "Test",
                    "first_name": "Test",
                    "last_name": "User",
                    "email": "test@mail",
                    "birthdate": "2020-01-01"
                }"#,
            ))
            .unwrap();

        let resp = app
            .oneshot(req)
            .await
            .unwrap();

        // extract the response body as bytes
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();

        // assert to check if user was not found
        assert_eq!(body, r#"User not found. Cannot update user details."#);
    }

}