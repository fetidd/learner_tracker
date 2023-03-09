use crate::{app::state::AppState, core::error::*, user::model::User};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use super::model::Record;

pub async fn handle_create_record(
    _state: AppState,
    _metric: Record,
    _user: User,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_records(_state: AppState, _user: User) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<Record>::new())))
}

pub async fn handle_get_record(
    _state: AppState,
    _id: Uuid,
    _user: User,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Record>::None)))
}

pub async fn handle_update_record(
    _state: AppState,
    _id: Uuid,
    _user: User,
    _metric: Record,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Record>::None)))
}

pub async fn handle_delete_record(_state: AppState, _id: Uuid, _user: User) -> Result<()> {
    Ok(())
}