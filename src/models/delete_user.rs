use serde::Deserialize;



#[derive(Deserialize)]
pub struct DeleteUser {
    pub username: String,
}