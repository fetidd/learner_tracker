mod common;
use crate::common::*;
use chrono::Utc;
use entity::{pupil::Model as Pupil, user::Model as User};
use http::StatusCode;
use lt_server::{auth::generate_auth_token, constant};
use rstest::*;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use uuid::Uuid;

async fn add_user(secret: &[u8], last_refresh: &str, db: &DatabaseConnection) -> User {
    let user = User {
        first_names: "Integration Test".into(),
        last_name: "User".into(),
        email_address: "test_user@integration.com".into(),
        hashed_password: "password".into(),
        years: "5,6".into(),
        secret: secret.to_vec(),
        last_refresh: last_refresh.parse().expect("parse last_refresh"),
    };
    entity::user::Entity::insert(<User as Into<entity::user::ActiveModel>>::into(
        user.clone(),
    ))
    .exec(db)
    .await
    .expect("insert test user");
    user
}

async fn add_pupils(db: &DatabaseConnection) {
    let pupils = vec![
        Pupil {
            id: Uuid::new_v4(),
            first_names: "first".into(),
            last_name: "student".into(),
            start_date: "2021-01-01".parse().unwrap(),
            end_date: "2027-01-01".parse().unwrap(),
            gender: "gender".into(),
            year: 6,
            active: true,
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
            active: true,
            ..Default::default()
        },
        Pupil {
            id: Uuid::new_v4(),
            first_names: "third".into(),
            last_name: "student".into(),
            start_date: "2021-01-01".parse().unwrap(),
            end_date: "2027-01-01".parse().unwrap(),
            gender: "gender".into(),
            year: 2,
            active: true,
            ..Default::default()
        },
    ];
    let to_insert: Vec<entity::pupil::ActiveModel> = pupils
        .clone()
        .into_iter()
        .map(entity::pupil::ActiveModel::from)
        .collect();
    entity::pupil::Entity::insert_many(to_insert)
        .exec(db)
        .await
        .expect("insert test pupils");
}

#[rstest]
async fn login_and_create_pupil(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let pupil = json!({
            "first_names": "first",
            "last_name": "last",
            "year": 6,
            "start_date": "2022-01-01",
            "end_date": "2024-01-01",
            "gender": "male",
            "more_able_and_talented": false,
            "english_as_additional_language": false,
            "free_school_meals": false,
            "additional_learning_needs": false,
            "looked_after_child": false,
            "active": false
    });
    add_user(&[127; 64], "2021-01-01T00:00:00", ctx.check_db()).await;
    let log_res = ctx
        .client()
        .post(constant::LOGIN_ENDPOINT)
        .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "password"}))
        .send()
        .await;
    assert_eq!(log_res.status(), StatusCode::OK);
    let res_json: HashMap<String, String> = log_res.json().await;
    let token = res_json.get("token").expect("auth token");
    let res = ctx
        .client()
        .post(constant::PUPILS_ENDPOINT)
        .json(&pupil)
        .header("Authorization", format!("Bearer {token}"))
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
async fn login_and_get_pupils(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    add_pupils(ctx.check_db()).await;
    add_user(&[127; 64], "2021-01-01T00:00:00", ctx.check_db()).await;
    let log_res = ctx
        .client()
        .post(constant::LOGIN_ENDPOINT)
        .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "password"}))
        .send()
        .await;
    assert_eq!(log_res.status(), StatusCode::OK);
    let res_json: HashMap<String, String> = log_res.json().await;
    let token = res_json.get("token").expect("auth token");
    let res = ctx
        .client()
        .get(constant::PUPILS_ENDPOINT)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let mut res_body: serde_json::Value = res.json().await;
    let mut pupils = res_body["pupils"].as_array_mut().expect("array of pupils");
    let ids: Vec<String> = pupils
        .iter_mut()
        .map(|mut p| {
            p.as_object_mut()
                .unwrap()
                .remove_entry("id")
                .unwrap()
                .1
                .to_string()
        })
        .collect();
    assert_eq!(ids.len(), 2);
    assert_eq!(pupils.len(), 2);
    let exp_pupils = vec![
        json!({
            "first_names": "first",
            "last_name": "student",
            "year": 6,
            "start_date": "2021-01-01",
            "end_date": "2027-01-01",
            "gender": "gender",
            "more_able_and_talented": false,
            "english_as_additional_language": false,
            "free_school_meals": false,
            "additional_learning_needs": false,
            "looked_after_child": false,
            "active": true
        }),
        json!({
            "first_names": "second",
            "last_name": "student",
            "year": 6,
            "start_date": "2021-01-01",
            "end_date": "2027-01-01",
            "gender": "gender",
            "more_able_and_talented": false,
            "english_as_additional_language": false,
            "free_school_meals": false,
            "additional_learning_needs": false,
            "looked_after_child": false,
            "active": true
        }),
    ];
    assert_eq!(*pupils, exp_pupils);
}

#[rstest]
async fn test_get_pupil_by_id(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let request_uuid = Uuid::new_v4();
    let pupils = vec![Pupil {
        id: request_uuid,
        first_names: "first".into(),
        last_name: "student".into(),
        start_date: "2021-01-01".parse().unwrap(),
        end_date: "2027-01-01".parse().unwrap(),
        gender: "gender".into(),
        year: 6,
        ..Default::default()
    }];
    let to_insert: Vec<entity::pupil::ActiveModel> = pupils
        .clone()
        .into_iter()
        .map(entity::pupil::ActiveModel::from)
        .collect();
    entity::pupil::Entity::insert_many(to_insert)
        .exec(ctx.check_db())
        .await
        .expect("adding pupils");
    add_user(&[127; 64], "2021-01-01T00:00:00", ctx.check_db()).await;
    let login = ctx
        .client()
        .post(constant::LOGIN_ENDPOINT)
        .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "password"}))
        .send()
        .await;
    assert_eq!(login.status(), StatusCode::OK);
    let token = login.json::<HashMap<String, String>>().await;
    let res = ctx
        .client()
        .get(&format!("{}/{request_uuid}", constant::PUPILS_ENDPOINT))
        .header("Authorization", format!("Bearer {}", token["token"]))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let res = res.json::<Value>().await;
    let res = res.as_object().unwrap();
    let act_pupils = res["pupils"].as_array().unwrap();
    assert_eq!(
        act_pupils[0],
        json!({
            "id": request_uuid,
            "first_names": "first",
            "last_name": "student",
            "start_date": "2021-01-01",
            "end_date": "2027-01-01",
            "gender": "gender",
            "year": 6,
            "active": false,
            "more_able_and_talented": false,
            "english_as_additional_language": false,
            "free_school_meals": false,
            "additional_learning_needs": false,
            "looked_after_child": false
        })
    );
}

#[rstest]
async fn test_get_users(#[future] mock_ctx: MockCtx) {
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
    let to_insert: Vec<entity::user::ActiveModel> = users
        .clone()
        .into_iter()
        .map(entity::user::ActiveModel::from)
        .collect();
    entity::user::Entity::insert_many(to_insert)
        .exec(ctx.check_db())
        .await
        .expect("inserting user");
    let res = ctx.client().get(constant::USERS_ENDPOINT).send().await;
    assert_eq!(res.status(), StatusCode::OK);
    let res_body: String = res.json().await;
    assert_eq!(res_body, "")
}

#[rstest]
async fn test_create_user(#[future] mock_ctx: MockCtx) {
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
        .post(constant::USERS_ENDPOINT)
        .json(&new_user_json)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let inserted = entity::user::Entity::find_by_id("test@test.com")
        .one(ctx.check_db())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(inserted.email_address, "test@test.com");
    assert_eq!(inserted.years, "2,3");
}
