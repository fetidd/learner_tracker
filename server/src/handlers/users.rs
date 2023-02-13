use crate::{
    app_state::{AppState},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::models::User;


pub async fn create_user( // TEST handlers need a refactor to make core app data/state a dependnecy to inject
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match user.save(state.database().as_ref()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"error": Option::<String>::None}))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": Some(error.to_string())}))),
    }
}

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse { // TEST
    match User::all_from_db(state.database().as_ref()).await {
        Ok(users) => (StatusCode::OK, Json(json!({"users": Some(users), "error": Option::<String>::None}))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"users": Option::<Vec<User>>::None, "error": Some(error.to_string())}))),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_utils::*;
    use crate::MockCtx;
    use entity::user::Model as User;
    use http::StatusCode;
    use rstest::*;
    use sea_orm::EntityTrait;

    #[rstest]
    async fn test_get_users(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let users = vec![
            User {
                first_names: "first".into(),
                last_name: "user".into(),
                email_address: "first_user@test.com".into(),
                hashed_password: "hashed_password".into(),
                years: "5,6".into(),
            },
            User {
                first_names: "second".into(),
                last_name: "user".into(),
                email_address: "second_user@test.com".into(),
                hashed_password: "hashed_password".into(),
                years: "2".into(),
            },
        ];
        let to_insert: Vec<entity::user::ActiveModel> = users
            .clone()
            .into_iter()
            .map(entity::user::ActiveModel::from)
            .collect();
        entity::user::Entity::insert_many(to_insert)
            .exec(ctx.check_db())
            .await
            .expect("inserting user");
        let res = ctx.client().get("/api/data/users").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let res: Vec<User> = res.json().await;
        assert_eq!(res, users);
    }
}
