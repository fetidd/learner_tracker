use mockall::automock;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppStateObj {
    database: Arc<DatabaseConnection>,
    secret: [u8; 64],
}

pub type AppState = Arc<dyn AppStateTrait + Send + Sync>;

impl AppStateObj {
    pub fn new(database: Arc<DatabaseConnection>, secret: [u8; 64]) -> Self {
        Self { database, secret }
    }
}

impl AppStateTrait for AppStateObj {
    fn database(&self) -> &Arc<DatabaseConnection> {
        &self.database
    }

    fn secret(&self) -> &[u8; 64] {
        &self.secret
    }
}

#[automock]
pub trait AppStateTrait {
    fn database(&self) -> &Arc<DatabaseConnection>;
    fn secret(&self) -> &[u8; 64];
}
