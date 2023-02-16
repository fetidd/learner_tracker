use crate::{
    error::{Error, ErrorKind, Result},
    models::User,
};
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub(crate) fn generate_auth_token(user: &User, secret: &[u8; 64]) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = AuthToken {
        email_address: user.email_address.to_owned(),
        exp: expiration as usize,
        first_names: user.first_names.to_owned(),
        last_name: user.last_name.to_owned(),
        years: user.years.to_owned(),
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret)).map_err(|e| Error {
        kind: ErrorKind::JWTTokenCreationError,
        message: e.to_string(),
    })
}

pub(crate) fn authorize_token(token: &str, secret: &[u8; 64]) -> Result<AuthToken> {
    Ok(decode::<AuthToken>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS512),
    )?
    .claims)
}

pub(crate) fn decode_token(token: &str) -> Result<AuthToken> {
    Ok(serde_json::from_str(&String::from_utf8(
        general_purpose::STANDARD_NO_PAD.decode(token.split('.').collect::<Vec<&str>>()[1])?,
    )?)?)
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthToken {
    pub(crate) email_address: String,
    pub(crate) exp: usize,
    pub(crate) first_names: String,
    pub(crate) last_name: String,
    pub(crate) years: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_generate_auth_token() {
        let secret: [u8; 64] = [127; 64];
        let user = User::new(
            "test",
            "user",
            "test@test.com",
            "hashedpassword",
            vec![3, 4],
        );
        let token = generate_auth_token(&user, &secret).expect("encoded token");
        let claims: AuthToken = serde_json::from_str(
            &String::from_utf8(
                general_purpose::STANDARD_NO_PAD
                    .decode(token.split('.').collect::<Vec<&str>>()[1])
                    .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(claims.email_address, user.email_address);
    }
}
