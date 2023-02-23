use axum_test_helper::TestClient;
use chrono::Utc;
use entity::{pupil::Model as Pupil, user::Model as User};
use lt_server::{
    core::constant,
    app::router::router,
    app::state::{AppStateTrait, MockAppStateTrait},
};
use migration::{Migrator, MigratorTrait};
use rstest::*;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

#[fixture]
pub async fn mock_ctx() -> MockCtx {
    let mut mock_state = MockAppStateTrait::new();
    let mock_db = Database::connect("sqlite::memory:").await.expect("connect to test database");
    Migrator::up(&mock_db, None).await.expect("migrate test database");
    let mock_db = Arc::new(mock_db);
    let check_db = Arc::clone(&mock_db);
    mock_state.expect_database().return_const(Arc::clone(&mock_db));
    let state: Arc<dyn AppStateTrait + Send + Sync> = Arc::new(mock_state);
    let app = router(Arc::clone(&state)).with_state(Arc::clone(&state));
    let client = TestClient::new(app);
    MockCtx { check_db, client }
}

pub struct MockCtx {
    check_db: Arc<DatabaseConnection>,
    client: TestClient,
}

impl MockCtx {
    pub fn check_db(&self) -> &DatabaseConnection {
        self.check_db.as_ref()
    }

    pub fn client(&self) -> &TestClient {
        &self.client
    }

    pub async fn login(&self) -> String {
        add_user(&[127; 64], "2021-01-01T00:00:00", self.check_db()).await;
        let login = self
            .client()
            .post(constant::LOGIN_ENDPOINT)
            .json(&json!({"email_address": "test_user@integration.com", "hashed_password": "password"}))
            .send()
            .await;
        assert_eq!(login.status(), http::StatusCode::OK);
        let token = login.json::<HashMap<String, String>>().await;
        token["token"].to_owned()
    }
}

pub async fn add_user(secret: &[u8], last_refresh: &str, db: &DatabaseConnection) -> User {
    let user = User {
        first_names: "Integration Test".into(),
        last_name: "User".into(),
        email_address: "test_user@integration.com".into(),
        hashed_password: "password".into(),
        years: "5,6".into(),
        secret: secret.to_vec(),
        last_refresh: last_refresh.parse().expect("parse last_refresh"),
    };
    entity::user::Entity::insert(<User as Into<entity::user::ActiveModel>>::into(user.clone()))
        .exec(db)
        .await
        .expect("insert test user");
    user
}

pub async fn add_pupils(db: &DatabaseConnection) -> Vec<&'static str> {
    let pupils = vec![
        Pupil {
            id: "a71ba7e6-0d07-4009-9515-414edd7f603a".parse().unwrap(),
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
            id: "f5e05074-4867-42de-8163-23e04a2a6e8b".parse().unwrap(),
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
            id: "8623e00f-b802-43d1-916e-8ac7a78ee001".parse().unwrap(),
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
    let to_insert: Vec<entity::pupil::ActiveModel> = pupils.clone().into_iter().map(entity::pupil::ActiveModel::from).collect();
    entity::pupil::Entity::insert_many(to_insert).exec(db).await.expect("insert test pupils");
    vec![
        "a71ba7e6-0d07-4009-9515-414edd7f603a",
        "f5e05074-4867-42de-8163-23e04a2a6e8b",
        "8623e00f-b802-43d1-916e-8ac7a78ee001",
    ]
}
