use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    pub site: String,
    pub username: String,
    pub password: String,
}
