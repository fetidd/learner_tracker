use crate::{app::state::AppState, core::error::*, record::model::*};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde_json::json;
use uuid::Uuid;