use base64::{engine::general_purpose, *};
use crate::error::Result;

use crate::users::User;

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
