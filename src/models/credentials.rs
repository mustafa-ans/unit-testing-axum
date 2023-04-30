use serde::{Deserialize};

#[derive(Deserialize, sqlx::FromRow)]

#[derive(Debug)]
pub struct User{
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, sqlx::FromRow)]
#[derive(Debug)]
pub struct First{
    pub first_name: String,
}