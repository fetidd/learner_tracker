use crate::{app_state::AppState, error::{LTResult, Error}};
use crate::models::User;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils;

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<RequestUser>,
) -> (StatusCode, Json<UsersResponse>) {
    if let Err(error) = req.validate() {
        (
            StatusCode::BAD_REQUEST,
            Json(UsersResponse {
                users: None,
                error: Some(error.to_string()),
            }),
        )
    } else {
        let user = User::new(&req.first_names, &req.last_name, &req.email_address, &req.hashed_password, req.years);
        match user.save(state.database().as_ref()).await {
            Ok(user) => (
                StatusCode::CREATED,
                Json(UsersResponse {
                    users: Some(vec![user.into()]),
                    error: None,
                }),
            ),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UsersResponse {
                    users: None,
                    error: Some(error.to_string()),
                }),
            ),
        }
    }
}

pub async fn get_users(State(state): State<AppState>) -> (StatusCode, Json<UsersResponse>) {
    match User::all_from_db(state.database().as_ref()).await {
        Ok(users) => (
            StatusCode::OK,
            Json(UsersResponse {
                users: Some(users.into_iter().map(ResponseUser::from).collect()),
                error: None,
            }),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UsersResponse {
                users: None,
                error: Some(error.to_string()),
            }),
        ),
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RequestUser {
    first_names: String,
    last_name: String,
    email_address: String,
    hashed_password: String,
    years: Vec<u32>
}

impl RequestUser {
    fn validate(&self) -> LTResult<()> {
        if self.first_names.is_empty() || self.last_name.is_empty() {
            Err(Error::InvalidRequest("names cannot be empty".into()))
        } else if self.hashed_password.is_empty() {
            Err(Error::InvalidRequest("password cannot be empty".into()))
        } else if self.years.is_empty() {
            Err(Error::InvalidRequest("must specify at least one year group".into()))
        } else if !utils::is_valid_email(&self.email_address) {
            Err(Error::InvalidRequest("email address is invalid".into()))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct ResponseUser {
    first_names: String,
    last_name: String,
    email_address: String,
    years: Vec<u32>
}

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            years: value.years,
        }
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct UsersResponse {
    #[serde(skip_serializing_if="Option::is_none")]
    users: Option<Vec<ResponseUser>>,
    #[serde(skip_serializing_if="Option::is_none")]
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use crate::MockCtx;
    use entity::user::{Model, Entity};
    use http::StatusCode;
    use rstest::*;
    use sea_orm::EntityTrait;
    use serde_json::json;

    #[rstest]
    async fn test_get_users(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let users = vec![
            Model {
                first_names: "first".into(),
                last_name: "user".into(),
                email_address: "first_user@test.com".into(),
                hashed_password: "hashed_password".into(),
                years: "5,6".into(),
            },
            Model {
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
        let res_body: UsersResponse = res.json().await;
        let res_users = res_body.users.expect("list of users");
        assert_eq!(res_users, users.into_iter().map(User::from).map(ResponseUser::from).collect::<Vec<ResponseUser>>())
    }

    #[rstest]
    async fn test_create_user(#[future] mock_ctx: MockCtx) {
        let ctx = mock_ctx.await;
        let new_user_json = json!({"first_names": "test", "last_name": "user", "email_address": "test@test.com", "hashed_password": "password", "years": vec![2,3]});
        let res = ctx.client().post("/api/data/users").json(&new_user_json).send().await;
        assert_eq!(res.status(), StatusCode::CREATED);
        let inserted = Entity::find_by_id("test@test.com").one(ctx.check_db()).await.unwrap().unwrap();
        assert_eq!(inserted.email_address, "test@test.com");
        assert_eq!(inserted.years, "2,3");
    }

    #[rstest]
    #[case("test", "user", "test@test.com", "password", vec![2,3],  Ok(()))]
    #[case("", "user", "test@test.com", "password", vec![2,3],      Err(Error::InvalidRequest("names cannot be empty".into())))]
    #[case("test", "", "test@test.com", "password", vec![2,3],      Err(Error::InvalidRequest("names cannot be empty".into())))]
    #[case("test", "user", "test@test.", "password", vec![2,3],     Err(Error::InvalidRequest("email address is invalid".into())))]
    #[case("test", "user", "testattest.com", "password", vec![2,3], Err(Error::InvalidRequest("email address is invalid".into())))]
    #[case("test", "user", "", "password", vec![2,3],               Err(Error::InvalidRequest("email address is invalid".into())))]
    #[case("test", "user", "test@test.com", "", vec![2,3],          Err(Error::InvalidRequest("password cannot be empty".into())))]
    #[case("test", "user", "test@test.com", "password", vec![],     Err(Error::InvalidRequest("must specify at least one year group".into())))]
    fn test_validate_request_user(#[case] first_names: String, #[case] last_name: String, #[case] email_address: String, #[case] hashed_password: String, #[case] years: Vec<u32>, #[case] exp: LTResult<()>) {
        let req = RequestUser {first_names, last_name, email_address, hashed_password, years};
        match exp {
            Ok(_) => assert!(req.validate().is_ok()),
            // Err(error) => assert_eq!(error, req.validate()) // TODO this needs to check error kind (new branch)
            Err(_) => assert!(req.validate().is_err())
        }
    }
}
