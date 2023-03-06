use crate::{app::state::AppState, core::error::*, pupil::model::*, user::model::*, metric::model::*};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde_json::json;
use uuid::Uuid;