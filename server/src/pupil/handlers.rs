use std::str::FromStr;

use crate::{
    core::error::*,
    pupil::{model::*},
    user::{model::*},
    app::state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub async fn create_pupil(State(state): State<AppState>, Json(pupil): Json<Pupil>) -> Result<StatusCode> {
    match pupil.insert(state.database().as_ref()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_pupils(State(state): State<AppState>, Extension(user): Extension<User>) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested all pupils");
    match Pupil::all_from_db(&user, state.database().as_ref()).await {
        Ok(pupils) => Ok(Json(json!(pupils))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_pupil_by_id(State(state): State<AppState>, Path(id): Path<String>, Extension(user): Extension<User>) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested pupil {id}");
    let id = Uuid::from_str(&id)?;
    match Pupil::one_from_db(&user, id, state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(json!(pupil))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn update_pupil(State(state): State<AppState>, Path(id): Path<String>, Extension(user): Extension<User>, Json(update): Json<PupilUpdate>) -> Result<Json<serde_json::Value>> {
    tracing::debug!("updating pupil {id}");
    let id = Uuid::from_str(&id)?;
    let mut pupil = Pupil::one_from_db(&user, id, state.database()).await?;
    pupil.set_from_update(update);
    match pupil.update(state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(json!(pupil))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn delete_pupil(State(state): State<AppState>, Path(id): Path<String>, Extension(user): Extension<User>) -> Result<StatusCode> {
    tracing::debug!("deleting pupil {id}");
    let id = Uuid::from_str(&id)?;
    let pupil = Pupil::one_from_db(&user, id, state.database()).await?;
    match pupil.delete(state.database().as_ref()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}
