use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Information {
    pub site: String,
    pub username: String,
    pub password: String,
}