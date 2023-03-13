use crate::{
    app::state::AppState,
    core::{constant, error::Result},
};
use axum::{
    extract::{State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    middleware::Next,
    response::Response,
};
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hyper::Request;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

// TODO add a way to reset the secret for every user that hasnt been refreshed in 24hours, check
// every 15 mins. Will need a last_refreshes field in yser table uodated in refresh_secret

pub fn generate_auth_token(user: &entity::user::Model) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(
            constant::AUTH_TOKEN_EXPIRY_MINUTES,
        ))
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
    encode(&header, &claims, &EncodingKey::from_secret(&user.secret))
        .map_err(|e| JWTTokenCreationError!(e.to_string()))
}

pub fn authorize_token(token: &str, secret: &[u8]) -> Result<AuthToken> {
    Ok(decode::<AuthToken>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS512),
    )?
    .claims)
}

pub fn decode_token(token: &str) -> Result<AuthToken> {
    match token.split('.').collect::<Vec<&str>>().get(1) {
        Some(claims) => {
            let decoded = general_purpose::STANDARD_NO_PAD.decode(*claims)?;
            let decoded_string = String::from_utf8(decoded)?;
            let token = serde_json::from_str(&decoded_string)?;
            Ok(token)
        }
        None => Err(DecodeError!()),
    }
}

pub async fn auth_service<B>(
    State(state): State<AppState>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let decoded = decode_token(auth_header.token())?;
    let user = entity::user::Entity::find_by_id(&decoded.email_address).one(state.database().as_ref()).await?;
    if let Some(user) = user {
        if authorize_token(auth_header.token(), &user.secret).is_ok() {
            request.extensions_mut().insert(user);
            let response = next.run(request).await;
            // create fresh token to pass in response
            Ok(response)
        } else {
            Err(InvalidJwt!())
        }
    } else {
        Err(UserDoesNotExist!())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub(crate) email_address: String,
    pub(crate) exp: usize,
    pub(crate) first_names: String,
    pub(crate) last_name: String,
    pub(crate) years: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_generate_auth_token() {
        let secret: [u8; 64] = [127; 64];
        let user = entity::user::Model {
            first_names: "test".into(),
            last_name: "user".into(),
            email_address: "test@test.com".into(),
            hashed_password: "pass".into(),
            years: vec![1,2],
            last_refresh: Utc::now().naive_local(),
            secret: crate::utils::functions::generate_secret().to_vec(),
        };
        user.secret = secret.into();
        let token = generate_auth_token(&user).expect("encoded token");
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
