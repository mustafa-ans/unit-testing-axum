use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub birthdate: NaiveDate,
    pub createdate: NaiveDateTime,
    pub updatedate: NaiveDateTime,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct First{
    pub first_name: String,
}