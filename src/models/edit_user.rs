use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birthdate: Option<NaiveDate>,
}

