use axum::Server;
use learner_tracker_server::{
    app_state::{AppState, AppStateObj},
    error::Result,
    router::router,
};
use migration::{seed_database, Migrator, MigratorTrait};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let current_env = std::env::var("ENVIRONMENT")?;
    tracing_subscriber::fmt::init();
    let db = Arc::new(sea_orm::Database::connect(std::env::var("DATABASE_URL")?).await?);
    if current_env == "dev" {
        tracing::debug!("running in development environment");
        Migrator::fresh(db.as_ref()).await?;
        seed_database(db.as_ref()).await;
    } else {
        Migrator::up(db.as_ref(), None).await?;
    }
    let app_state: AppState = Arc::new(AppStateObj::new(db));
    let address: SocketAddr = std::env::var("SERVER_ADDR")?.parse()?;
    Server::bind(&address)
        .serve(
            router()
                .layer(TraceLayer::new_for_http())
                .with_state(app_state)
                .into_make_service(),
        )
        .await?;
    Ok(())
}
