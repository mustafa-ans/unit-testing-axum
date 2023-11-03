#![allow(unused_imports)]
use axum::{
    extract::Extension,
    routing::{get,post, delete, put, delete_service},
    Router,
    handler::Handler
};

use controllers::{get_user::get_user_handler, add_user::create_user};
use sqlx::postgres::PgPoolOptions;

use crate::controllers::{edit_user::update_user, delete_user::delete_user};

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
        .route("/",get(|| async { "Sample API application for unit testing" }))
        .route("/get-users",get(get_user_handler))
        .route("/add-users",post(create_user))
        .route("/update-users",put(update_user))
        .route("/delete-users",delete(delete_user))
        .layer(Extension(pool));
    
    

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0,0,0,0],3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");

}