use chrono::NaiveDate;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birthdate: Option<NaiveDate>,
}