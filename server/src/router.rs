use crate::{
    handlers::{login::*, pupils::*, users::*},
    state::AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post, put},
    Router,
};
use hyper::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn router(state: AppState) -> Router<AppState> {
    let auth_router = Router::new().route("/login", post(login_handler)).route("/logout", get(logout_handler));
    let pupils_router = Router::new()
        .route("/", get(get_pupils).put(create_pupil))
        .route("/:id", get(get_pupil_by_id).post(update_pupil).delete(delete_pupil));
    let users_router = Router::new().route("/", put(create_user).get(get_users));
    let data_router = Router::new().nest("/pupils", pupils_router).nest("/users", users_router);
    let cors_layer = CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]).allow_origin(Any); // TODOSERVER this needs to only be the actual url (research this!!)

    Router::new().nest(
        "/api",
        Router::new()
            .nest("/data", data_router)
            .layer(from_fn_with_state(Arc::clone(&state), crate::auth::auth_service))
            .nest("/auth", auth_router)
            .layer(cors_layer),
    )
}
