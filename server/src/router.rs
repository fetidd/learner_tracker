use crate::{
    app_state::AppState,
    handlers::{login::*, pupils::*, users::*},
};
use axum::{
    routing::{get, post},
    Router,
};
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn router() -> Router<AppState> {
    let auth_router = Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler));
    let pupils_router = Router::new()
        .route("/", get(get_pupils).post(create_pupil))
        .route("/:id", get(get_pupil_by_id));
    let users_router = Router::new().route("/", post(create_user).get(get_users));
    let data_router = Router::new()
        .nest("/pupils", pupils_router)
        .nest("/users", users_router);
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any); // TODOSERVER this needs to only be the actual url (research this!!)

    Router::new().nest(
        "/api",
        Router::new()
            .nest("/data", data_router)
            .nest("/auth", auth_router)
            .layer(cors_layer),
    )
}
