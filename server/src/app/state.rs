use mockall::automock;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppStateObj {
    database: Arc<DatabaseConnection>,
}

pub type AppState = Arc<dyn AppStateTrait + Send + Sync>;

impl AppStateObj {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }
}

impl AppStateTrait for AppStateObj {
    fn database(&self) -> &Arc<DatabaseConnection> {
        &self.database
    }
}

#[automock]
pub trait AppStateTrait {
    fn database(&self) -> &Arc<DatabaseConnection>;
}
