use axum::{
    extract::Extension,
    routing::{get,post},
    Router};

use controllers::insert::get_user_handler;
use sqlx::postgres::PgPoolOptions;

mod controllers;
mod models;

#[tokio::main]
async fn main(){
    let durl = std::env::var("DATABASE_URL_ONLINE").expect("set DATABASE_URL env variable");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to make connection");

    let app = Router::new()
        .route("/user",get(get_user_handler))
        .layer(Extension(pool));
    
    

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0,0,0,0],3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}