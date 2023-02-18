use axum::Server;
use lt_server::{
    app_state::{AppState, AppStateObj},
    error::Result,
    router::router,
    utils,
};
use migration::{seed_database, Migrator, MigratorTrait};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let current_env = std::env::var("ENVIRONMENT")?;
    let log_fmt = tracing_subscriber::fmt::format() // TODO move to log module
        .pretty()
        .without_time()
        .with_target(true)
        .with_thread_names(false);
    tracing_subscriber::fmt()
        .event_format(log_fmt)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let secret: [u8; 64] = utils::generate_secret();
    let db = Arc::new(sea_orm::Database::connect(std::env::var("DATABASE_URL")?).await?);
    if current_env == "dev" {
        tracing::debug!("running in development environment");
        Migrator::fresh(db.as_ref()).await?;
        seed_database(db.as_ref()).await;
    } else {
        Migrator::up(db.as_ref(), None).await?;
    }
    let app_state: AppState = Arc::new(AppStateObj::new(db, secret));
    let address: SocketAddr = std::env::var("SERVER_ADDR")?.parse()?;
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
