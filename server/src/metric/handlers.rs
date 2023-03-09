use crate::{app::state::AppState, core::error::*, user::model::User};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use super::model::Metric;

pub async fn handle_create_metric(
    _state: AppState,
    _metric: Metric,
    _user: User,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_metrics(_state: AppState, _user: User) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<Metric>::new())))
}

pub async fn handle_get_metric(
    _state: AppState,
    _id: Uuid,
    _user: User,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Metric>::None)))
}

pub async fn handle_update_metric(
    _state: AppState,
    _id: Uuid,
    _user: User,
    _metric: Metric,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Metric>::None)))
}

pub async fn handle_delete_metric(_state: AppState, _id: Uuid, _user: User) -> Result<()> {
    Ok(())
}
