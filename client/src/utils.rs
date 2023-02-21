use base64::{engine::general_purpose, *};

use crate::models::User;

pub fn decode_auth_token(token: String) -> Result<User, String> {
    match token.split('.').collect::<Vec<&str>>().get(1) {
        Some(claims) => {
            let decoded = general_purpose::STANDARD_NO_PAD.decode(*claims).map_err(|_| String::from("failed decoding from base64"))?;
            let decoded_string = String::from_utf8(decoded).map_err(|_| String::from("failed decoding into string"))?;
            let user: User = serde_json::from_str(&decoded_string).map_err(|_| String::from("failed deserializing into target type"))?;
            Ok(user)
        }
        None => Err("token had too few parts".into()),
    }
}
