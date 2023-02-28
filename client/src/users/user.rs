use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub first_names: String,
    pub last_name: String,
    pub email_address: String,
    pub years: Vec<u32>,
}