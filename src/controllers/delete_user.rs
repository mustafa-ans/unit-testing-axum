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


#[cfg(test)]
mod tests {
    use axum::{http::{Request, Method}, body::Body, routing::{get, post, delete} , Router};
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
    async fn test_delete_user_route(){
        let pool = create_connection_pool().await;
        let app = Router::new()
            .route("/delete_user", delete(delete_user))
            .layer(Extension(pool));

        // send a request to the route with query parameters

        let req = Request::builder()
            .method(Method::DELETE)
            .uri("/delete_user?username=johndoe87")
            .body(Body::empty())
            .unwrap();

        let res = app
            .oneshot(req)
            .await
            .unwrap();

        // check the response body is as expected
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert_eq!(body, r#"{"message":"User 'johndoe87' deleted successfully."}"#);
    }

    #[tokio::test]
    async fn test_delete_user_route_user_not_found(){
        let pool = create_connection_pool().await;
        let app = Router::new()
            .route("/delete_user", delete(delete_user))
            .layer(Extension(pool));

        // send a request to the route with query parameters

        let req = Request::builder()
            .method(Method::DELETE)
            .uri("/delete_user?username=TestUser")
            .body(Body::empty())
            .unwrap();

        let res = app
            .oneshot(req)
            .await
            .unwrap();

        // check the response body is as expected
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert_eq!(body, r#"User not found. Cannot delete user."#);
    }

}