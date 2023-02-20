use crate::common::{self, mock_ctx, MockCtx};
use http::StatusCode;
use lt_server::constant;
use regex::Regex;
use rstest::*;
use serde_json::{json, Value};

#[rstest]
async fn successful_login(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    common::add_user(&[127; 64], "2022-01-01T00:00:00", ctx.check_db()).await;
    let res = ctx
        .client()
        .post(constant::LOGIN_ENDPOINT)
        .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "password"}))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let res_body = res.json::<Value>().await;
    let res_body = res_body.as_object().expect("token response");
    let token = res_body["token"].clone();
    let re = Regex::new(r"^ey.+\.ey.+\..+$").unwrap(); // check it's a jwt
    assert!(re.is_match(token.as_str().expect("token")));
}

#[rstest]
async fn unsuccessful_login_bad_password(#[future] mock_ctx: MockCtx) {
    let ctx = mock_ctx.await;
    common::add_user(&[127; 64], "2022-01-01T00:00:00", ctx.check_db()).await;
    let res = ctx
        .client()
        .post(constant::LOGIN_ENDPOINT)
        .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "thisishtewrongpassword"}))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let res_body = res.json::<Value>().await;
    let res_body = res_body.as_object().expect("error response");
    let error = res_body["error"].clone();
    assert_eq!(error, "INVALID CREDENTIALS");
}
