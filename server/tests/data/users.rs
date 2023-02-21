use crate::common::*;
use chrono::Utc;
use http::StatusCode;
use lt_server::constant;
use rstest::*;
use sea_orm::EntityTrait;
use serde_json::json;

#[rstest]
async fn login_and_get_users(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let users = vec![
        entity::user::Model {
            first_names: "first".into(),
            last_name: "user".into(),
            email_address: "first_user@test.com".into(),
            hashed_password: "hashed_password".into(),
            years: "5,6".into(),
            secret: vec![127; 64],
            last_refresh: Utc::now().naive_utc(),
        },
        entity::user::Model {
            first_names: "second".into(),
            last_name: "user".into(),
            email_address: "second_user@test.com".into(),
            hashed_password: "hashed_password".into(),
            years: "2".into(),
            secret: vec![127; 64],
            last_refresh: Utc::now().naive_utc(),
        },
    ];
    let to_insert: Vec<entity::user::ActiveModel> = users.clone().into_iter().map(entity::user::ActiveModel::from).collect();
    entity::user::Entity::insert_many(to_insert).exec(ctx.check_db()).await.expect("inserting user");
    let res = ctx
        .client()
        .get(constant::USERS_ENDPOINT)
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let mut res_body: serde_json::Value = res.json().await;
    let users = res_body["users"].as_array_mut().expect("array of users");
    assert_eq!(
        *users,
        vec![
            json!({
                "first_names": "first",
                "last_name": "user",
                "email_address": "first_user@test.com",
                "years": vec![5,6]
            }),
            json!({
                "first_names": "second",
                "last_name": "user",
                "email_address": "second_user@test.com",
                "years": vec![2]
            }),
            json!({
                "first_names": "Integration Test",
                "last_name": "User",
                "email_address": "test_user@integration.com",
                "years": vec![5,6]
            }),
        ]
    );
}

#[rstest]
async fn login_and_create_user(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let new_user_json = json!({
        "first_names": "test",
        "last_name": "user",
        "email_address": "test@test.com",
        "hashed_password": "password",
        "years": vec![2,3]
    });
    let res = ctx
        .client()
        .put(constant::USERS_ENDPOINT)
        .json(&new_user_json)
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let inserted = entity::user::Entity::find_by_id("test@test.com").one(ctx.check_db()).await.unwrap().unwrap();
    assert_eq!(inserted.email_address, "test@test.com");
    assert_eq!(inserted.years, "2,3");
}
