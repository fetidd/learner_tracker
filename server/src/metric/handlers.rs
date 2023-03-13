use crate::{app::state::AppState, core::error::*};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;
use entity::*;

pub async fn handle_create_metric(
    _state: AppState,
    _metric: metric::Model,
    _user: user::Model,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_metrics(_state: AppState, _user: user::Model) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<metric::Model>::new())))
}

pub async fn handle_get_metric(
    _state: AppState,
    _id: Uuid,
    _user: user::Model,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<metric::Model>::None)))
}

pub async fn handle_update_metric(
    _state: AppState,
    _id: Uuid,
    _user: user::Model,
    _metric: metric::Model,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<metric::Model>::None)))
}

pub async fn handle_delete_metric(_state: AppState, _id: Uuid, _user: user::Model) -> Result<()> {
    Ok(())
}
