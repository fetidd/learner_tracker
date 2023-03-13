use crate::{app::state::AppState, auth::token::*, core::error::*};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use tracing::debug;
use entity::user;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    debug!("login request for {}", login_req.email_address);
    let user = get_and_validate_user(
        login_req.email_address,
        login_req.hashed_password,
        state.database(),
    )
    .await?;
    debug!("generating auth token");
    let auth_token = generate_auth_token(&user)?;
    debug!("responding with token for user {}", user.email_address);
    Ok(Json(LoginResponse { token: auth_token }))
}

pub async fn logout_handler(
    State(state): State<AppState>,
    TypedHeader(auth_token): TypedHeader<Authorization<Bearer>>,
) -> Result<StatusCode> {
    let decoded = decode_token(auth_token.token())?;
    debug!("logout request for {}", decoded.email_address);
    let user = user::Entity::find_by_id(&decoded.email_address).one(state.database().as_ref()).await?;
    // user.refresh_secret(state.database()).await?; // TODO refreshing secret
    debug!("refreshed secret for {}", decoded.email_address);
    if let Some(user) = user {
        if authorize_token(auth_token.token(), &user.secret).is_ok() {
            Ok(StatusCode::OK)
        } else {
            Err(InvalidJwt!())
        }
    } else {
        Err(UserDoesNotExist!())
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email_address: String,
    hashed_password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

async fn get_and_validate_user(
    email: String,
    pass: String,
    db: &DatabaseConnection,
) -> Result<user::Model> {
    let user: Option<user::Model> = user::Entity::find_by_id(&email).one(db).await.map_err(|e| Error::from(e))?;
    if let Some(user) = user {
        if pass == user.hashed_password {
            Ok(user)
        } else {
            Err(InvalidCredentials!())
        }
    } else {
        Err(UserDoesNotExist!())
    }
}
