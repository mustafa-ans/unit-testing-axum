use axum::{Extension,Json};
use serde_json::{json,Value};
use sqlx::PgPool;
use crate::models;

use ::axum::Router;

pub async fn get_user_handler(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    let result = sqlx::query_as::<_, models::get_user::User>(
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
    
    // return Ok(Json(json!({})));
}


#[cfg(test)]
mod tests{
    use axum::{http::{Request, Method}, body::Body};
    use sqlx_core::postgres::PgPoolOptions;
    use ::axum::routing::get;
    use tower::util::ServiceExt;
    use super::*;

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
    async fn test_get_user_handler_route(){

        let pool = create_connection_pool().await;

        let app = Router::new()
            .route("/user",get(get_user_handler))
            .layer(Extension(pool));     

        let request = Request::builder()
            .uri("/user")
            .method(Method::GET)  // Use the appropriate HTTP method
            .body(Body::empty())  // Provide a valid request body
            .unwrap();

        let response = app
            .oneshot(request)
            .await
            .unwrap();

        assert_eq!(response.status(),200); 

    }

}