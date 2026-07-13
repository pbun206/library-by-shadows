use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}
