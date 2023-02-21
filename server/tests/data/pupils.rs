use crate::common::*;
use entity::pupil::Model as Pupil;
use http::StatusCode;
use lt_server::constant;
use rstest::*;
use sea_orm::EntityTrait;
use serde_json::{json, Value};
use uuid::Uuid;

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
    let res = ctx
        .client()
        .put(constant::PUPILS_ENDPOINT)
        .json(&pupil)
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let inserted = &entity::pupil::Entity::find().all(ctx.check_db()).await.unwrap()[0];
    let pupil = pupil.as_object().unwrap();
    assert_eq!(inserted.first_names, pupil["first_names"]);
    assert_eq!(inserted.last_name, pupil["last_name"]);
}

#[rstest]
async fn login_and_get_pupils(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    add_pupils(ctx.check_db()).await;
    let res = ctx
        .client()
        .get(constant::PUPILS_ENDPOINT)
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let mut res_body: serde_json::Value = res.json().await;
    let pupils = res_body["pupils"].as_array_mut().expect("array of pupils");
    let ids: Vec<String> = pupils.iter_mut().map(|p| p.as_object_mut().unwrap().remove_entry("id").unwrap().1.to_string()).collect();
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
async fn login_and_get_pupil_by_id(#[future] mock_ctx: MockCtx) {
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
    let to_insert: Vec<entity::pupil::ActiveModel> = pupils.clone().into_iter().map(entity::pupil::ActiveModel::from).collect();
    entity::pupil::Entity::insert_many(to_insert).exec(ctx.check_db()).await.expect("adding pupils");
    let res = ctx
        .client()
        .get(&format!("{}/{request_uuid}", constant::PUPILS_ENDPOINT))
        .header("Authorization", format!("Bearer {}", ctx.login().await))
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
async fn login_and_update_pupil(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let ids = add_pupils(ctx.check_db()).await;
    let res = ctx
        .client()
        .post(&format!("{}/{}", constant::PUPILS_ENDPOINT, ids[0]))
        .json(&json!({
            "free_school_meals": true,
            "looked_after_child": true,
            "last_name": "newname"
        }))
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let updated_pupil = entity::pupil::Entity::find_by_id(ids[0].parse::<Uuid>().expect("parsed uuid"))
        .one(ctx.check_db())
        .await
        .expect("successful query")
        .expect("found updated pupil");
    assert_eq!(
        updated_pupil,
        Pupil {
            id: ids[0].parse().unwrap(),
            first_names: "first".into(),
            last_name: "newname".into(),
            start_date: "2021-01-01".parse().unwrap(),
            end_date: "2027-01-01".parse().unwrap(),
            gender: "gender".into(),
            year: 6,
            active: true,
            looked_after_child: true,
            free_school_meals: true,
            ..Default::default()
        }
    );
}

#[rstest]
async fn login_and_delete_pupil(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    let ids = add_pupils(ctx.check_db()).await;
    let res = ctx
        .client()
        .delete(&format!("{}/{}", constant::PUPILS_ENDPOINT, ids[0]))
        .header("Authorization", format!("Bearer {}", ctx.login().await))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let updated_pupil = entity::pupil::Entity::find_by_id(ids[0].parse::<Uuid>().expect("parsed uuid"))
        .one(ctx.check_db())
        .await
        .expect("successful query");
    assert_eq!(updated_pupil, None);
}
