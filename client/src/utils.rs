use base64::{engine::general_purpose, *};
use gloo_storage::{SessionStorage, Storage};
use crate::error::Result;

use crate::{models::User, constant};

pub fn decode_auth_token(token: String) -> Result<User> {
    match token.split('.').collect::<Vec<&str>>().get(1) {
        Some(claims) => {
            let decoded = general_purpose::STANDARD_NO_PAD.decode(*claims)?;
            let decoded_string = String::from_utf8(decoded)?;
            let user: User = serde_json::from_str(&decoded_string)?;
            Ok(user)
        }
        None => Err(DecodeError!()),
    }
}

pub fn get_current_token() -> Result<String> {
    Ok(SessionStorage::get::<String>(constant::AUTH_TOKEN_STORAGE_KEY)?)
}
