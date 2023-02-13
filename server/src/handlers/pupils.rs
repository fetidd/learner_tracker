use crate::{app_state::AppState};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use serde_json::json;
use crate::models;

pub async fn create_pupil(
    State(state): State<AppState>,
    Json(pupil): Json<models::Pupil>,
) -> impl IntoResponse {
    match models::Pupil::from(pupil).save(state.database().as_ref()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"error": Option::<String>::None}))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": Some(error.to_string())}))),
    }
}

pub async fn get_pupils(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!("requested all pupils");
    match models::Pupil::all_from_db(state.database().as_ref()).await {
        Ok(pupils) => (StatusCode::OK, Json(json!({"pupils": Some(pupils), "error": Option::<String>::None}))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"pupils": Option::<Vec<models::Pupil>>::None, "error": Some(error.to_string())}))),
    }
}

pub async fn get_pupil_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match models::Pupil::one_from_db(id, state.database().as_ref()).await {
        Ok(pupil) => (StatusCode::OK, Json(json!({"pupils": Some(pupil), "error": Option::<String>::None}))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"pupil": Option::<Vec<models::Pupil>>::None, "error": Some(error.to_string())}))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::test_utils::*, constant};
    use crate::MockCtx;
    use entity::pupil::Model as Pupil;
    use http::StatusCode;
    use rstest::*;
    use sea_orm::EntityTrait;
    use serde_json::json;
    use uuid::Uuid;

    #[rstest]
    async fn test_create_pupil(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let pupil = json!({
                "first_names": "first", "last_name": "last", "year": 6, "start_date": "2022-01-01", "end_date": "2024-01-01", "gender": "male", "more_able_and_talented": false, "english_as_additional_language": false, "free_school_meals": false, "additional_learning_needs": false, "looked_after_child": false, "active": false
        });
        let res = ctx
            .client()
            .post(constant::PUPILS_ENDPOINT)
            .json(&pupil)
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::CREATED);
        let inserted = &entity::pupil::Entity::find()
            .all(ctx.check_db())
            .await
            .unwrap()[0];
        let pupil = pupil.as_object().unwrap();
        assert_eq!(inserted.first_names, pupil["first_names"]);
        assert_eq!(inserted.last_name, pupil["last_name"]);
    }

    #[rstest]
    async fn test_get_pupils(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let pupils = vec![
            Pupil {
                id: Uuid::new_v4(),
                first_names: "first".into(),
                last_name: "student".into(),
                start_date: "2021-01-01".parse().unwrap(),
                end_date: "2027-01-01".parse().unwrap(),
                gender: "gender".into(),
                year: 6,
                ..Default::default()
            },
            Pupil {
                id: Uuid::new_v4(),
                first_names: "second".into(),
                last_name: "student".into(),
                start_date: "2021-01-01".parse().unwrap(),
                end_date: "2027-01-01".parse().unwrap(),
                gender: "gender".into(),
                year: 6,
                ..Default::default()
            },
            Pupil {
                id: Uuid::new_v4(),
                first_names: "second".into(),
                last_name: "student".into(),
                start_date: "2021-01-01".parse().unwrap(),
                end_date: "2027-01-01".parse().unwrap(),
                gender: "gender".into(),
                year: 6,
                ..Default::default()
            },
        ];
        let to_insert: Vec<entity::pupil::ActiveModel> = pupils
            .clone()
            .into_iter()
            .map(entity::pupil::ActiveModel::from)
            .collect();
        entity::pupil::Entity::insert_many(to_insert)
            .exec(ctx.check_db())
            .await
            .expect("adding pupils");
        let res = ctx.client().get(constant::PUPILS_ENDPOINT).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let response_pupils: Vec<Pupil> = res.json().await;
        assert_eq!(pupils, response_pupils);
    }

    #[rstest]
    async fn test_get_pupil_by_id(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let request_uuid = Uuid::new_v4();
        let pupils = vec![
            Pupil {
                id: request_uuid,
                first_names: "first".into(),
                last_name: "student".into(),
                start_date: "2021-01-01".parse().unwrap(),
                end_date: "2027-01-01".parse().unwrap(),
                gender: "gender".into(),
                year: 6,
                ..Default::default()
            },
            Pupil {
                id: Uuid::new_v4(),
                first_names: "second".into(),
                last_name: "student".into(),
                start_date: "2021-01-01".parse().unwrap(),
                end_date: "2027-01-01".parse().unwrap(),
                gender: "gender".into(),
                year: 6,
                ..Default::default()
            },
        ];
        let to_insert: Vec<entity::pupil::ActiveModel> = pupils
            .clone()
            .into_iter()
            .map(entity::pupil::ActiveModel::from)
            .collect();
        entity::pupil::Entity::insert_many(to_insert)
            .exec(ctx.check_db())
            .await
            .expect("adding pupils");
        let res = ctx
            .client()
            .get(&format!("{}/{request_uuid}", constant::PUPILS_ENDPOINT))
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        let response_pupil: Pupil = res.json().await;
        assert_eq!(response_pupil, pupils[0]);
    }
}
