use crate::{
    error::*,
    models::{pupil::PupilUpdate, Pupil, User},
    state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
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

pub async fn get_pupils(State(state): State<AppState>, Extension(user): Extension<User>) -> Result<Json<PupilsResponse>> {
    tracing::debug!("requested all pupils");
    match Pupil::all_from_db(&user, state.database().as_ref()).await {
        Ok(pupils) => Ok(Json(PupilsResponse { pupils })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_pupil_by_id(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(user): Extension<User>) -> Result<Json<PupilsResponse>> {
    tracing::debug!("requested pupil {id}");
    match Pupil::one_from_db(&user, id, state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(PupilsResponse { pupils: vec![pupil] })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn update_pupil(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(user): Extension<User>, Json(update): Json<PupilUpdate>) -> Result<Json<PupilsResponse>> {
    tracing::debug!("updating pupil {id}");
    let mut pupil = Pupil::one_from_db(&user, id, state.database()).await?;
    pupil.set_from_update(update);
    match pupil.update(state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(PupilsResponse { pupils: vec![pupil] })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PupilsResponse {
    pupils: Vec<Pupil>,
}

pub async fn delete_pupil(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(user): Extension<User>) -> Result<StatusCode> {
    tracing::debug!("deleting pupil {id}");
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
