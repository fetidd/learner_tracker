use axum::Server;
use lt_server::{
    error::Result,
    log::start_log,
    router::router,
    state::{AppState, AppStateObj},
    utils,
};
use migration::{seed_database, Migrator, MigratorTrait};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    start_log();
    let current_env = std::env::var("ENVIRONMENT")?;
    let secret: [u8; 64] = utils::generate_secret();
    let db = Arc::new(sea_orm::Database::connect(std::env::var("DATABASE_URL")?).await?);
    if current_env == "dev" {
        tracing::debug!("running in development environment");
        Migrator::fresh(db.as_ref()).await?;
        seed_database(db.as_ref()).await;
        tracing::debug!("database refreshed and seeded");
    } else {
        Migrator::up(db.as_ref(), None).await?;
    }
    let app_state: AppState = Arc::new(AppStateObj::new(db, secret));
    let address: SocketAddr = std::env::var("SERVER_ADDR")?.parse()?;
    tracing::debug!("listening on {address}");
    Server::bind(&address)
        .serve(
            router(Arc::clone(&app_state))
                .layer(TraceLayer::new_for_http())
                .with_state(app_state)
                .into_make_service(),
        )
        .await?;
    Ok(())
}
