use lazy_static::lazy_static;
use rand::RngCore;
use regex::Regex;

pub fn generate_secret() -> [u8; 64] {
    let mut secret: [u8; 64] = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut secret);
    secret
}

pub fn is_valid_email(email: &str) -> bool {
    lazy_static! {
        static ref EMAIL_REGEX: Regex =
            Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap();
    }
    EMAIL_REGEX.is_match(email)
}

#[cfg(test)]
pub mod test_utils {
    use crate::{app_state::{MockAppStateTrait, AppStateTrait}, router::router};
    use axum_test_helper::TestClient;
    use migration::{Migrator, MigratorTrait};
    use rstest::*;
    use sea_orm::{Database, DatabaseConnection};
    use std::sync::Arc;

    #[fixture]
    pub async fn mock_ctx() -> MockCtx {
        let mut mock_state = MockAppStateTrait::new();
        let mock_db = Database::connect("sqlite::memory:")
            .await
            .expect("connect to test database");
        Migrator::up(&mock_db, None)
            .await
            .expect("migrate test database");
        let mock_db = Arc::new(mock_db);
        let check_db = Arc::clone(&mock_db);
        mock_state
            .expect_database()
            .return_const(Arc::clone(&mock_db));
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
    }
}
