use axum::{Extension,Json};
use chrono::NaiveDate;
use serde_json::{json,Value};
use sqlx::PgPool;
use crate::models::{self, add_user::NewUser};




pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, String> {
    // Check if the username already exists
    let username_exists = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM Users WHERE Username = $1",
    )
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        let err_message = format!("Error checking if username exists, error returned from the database: \n{}", err);
        println!("{}",&err_message);
        err_message
    })?;

    if username_exists > 0 {
        return Err("Username already exists. Please choose a unique username.".to_string());
    }

    // If the username is unique, insert the new user
    sqlx::query(
        r#"
        INSERT INTO Users (Username, FirstName, LastName, Email, BirthDate)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&new_user.username)
    .bind(&new_user.first_name)
    .bind(&new_user.last_name)
    .bind(&new_user.email)
    .bind(&new_user.birthdate)
    .execute(&pool)
    .await
    .map_err(|err| {
        let err_message = format!("Error inserting new user into the database: \n{}", err);
        println!("{}",&err_message);
        err_message
    })?;

    Ok(Json(json!({"message": "User registered successfully."})))
}


// extract logic for checking username uniqueness

async fn _check_username_uniqueness(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, String> {
    // Check if the username already exists
    let username_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM Users WHERE Username = $1",
    )
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        let err_message = format!("Error checking if username exists, error returned from the database: \n{}", err);
        println!("{}",&err_message);
        err_message
    })?;

    if username_exists > 0 {
        return Err("Username already exists. Please choose a unique username.".to_string());
    }

    Ok(Json(json!({"message": "Username is unique."})))
}


#[cfg(test)]
mod tests{
    use axum::{http::{Request, Method}, body::Body, routing::{get, post} , Router};
    use sqlx_core::postgres::PgPoolOptions;
    use tower::util::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn check_database_connectivity(){
        let durl = std::env::var("DATABASE_URL_ONLINE").expect("set DATABASE_URL env variable");

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
    async fn test_create_user_route(){
        let _pool = create_connection_pool().await;
        let app = post(create_user);

        let req = Request::builder()
            .method(Method::POST)
            .uri("/add-users")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "username": "testuser",
                    "first_name": "Test",
                    "last_name": "User",
                    "email": "",
                    "birthdate": "2020-01-01"
                }"#,
            ))
            .unwrap();

        let response = app
            .oneshot(req)
            .await
            .unwrap();
        
        assert_eq!(response.status(),200);
}

    #[tokio::test]
    async fn test_username_uniqueness(){

        let pool = create_connection_pool().await;
        
        let app = Router::new()
            .route("/check-username",post(_check_username_uniqueness))
            .layer(Extension(pool));

        let req = Request::builder()
            .method(Method::POST)
            .uri("/check-username")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "username": "johndoe87"
                }"#,
            ))
            .unwrap();

        let response = app
            .oneshot(req)
            .await
            .unwrap();

        // assert_eq!(response.status(),200);

        // assert the username is unique from the response 
        let body_bytes = hyper::body::to_bytes(response.into_body())
            .await
            .expect("Failed to read response body");
    
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        let body: Value = serde_json::from_str(&body_str)
            .expect("Failed to parse JSON");

        println!("{:?}",body);
        assert_eq!(body["message"], "Username is unique.");



    }
}