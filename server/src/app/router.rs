use crate::{
    app::state::AppState,
    auth::{handlers::*, token::*},
    metric::handlers::*,
    pupil::handlers::*,
    record::handlers::*,
    user::handlers::*,
};
use axum::{
    extract::{Path, State},
    middleware::from_fn_with_state,
    routing::{get, post, put},
    Extension, Json, Router,
};
use hyper::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn router(state: AppState) -> Router<AppState> {
    let auth_router = Router::new()
        .route("/login", post(login_handler))
        .route("/logout", get(logout_handler));

    let pupils_router = Router::new()
        .route(
            "/",
            get(|State(state), Extension(user)| handle_get_pupils(state, user))
                .put(|State(state), Json(pupil)| handle_create_pupil(state, pupil)),
        )
        .route(
            "/:id",
            get(|State(state), Path(id), Extension(user)| handle_get_pupil(state, id, user))
                .post(|State(state), Path(id), Extension(user), Json(update)| {
                    handle_update_pupil(state, id, user, update)
                })
                .delete(|State(state), Path(id), Extension(user)| {
                    handle_delete_pupil(state, id, user)
                }),
        );

    let metrics_router = Router::new()
        .route(
            "/",
            get(|State(state), Extension(user)| handle_get_metrics(state, user))
            .put(|State(state), Extension(user), Json(metric)| handle_create_metric(state, metric, user)),
        )
        .route(
            "/:id",
            get(|State(state), Path(id), Extension(user)| handle_get_metric(state, id, user))
            .post(|State(state), Path(id), Extension(user), Json(update)| handle_update_metric(state, id, user, update))
            .delete(|State(state), Path(id), Extension(user)| handle_delete_metric(state, id, user))
        );
    let records_router = Router::new()
        .route(
            "/",
            get(|State(state), Extension(user)| handle_get_records(state, user))
            .put(|State(state), Extension(user), Json(metric)| handle_create_record(state, metric, user)),
        )
        .route(
            "/:id",
            get(|State(state), Path(id), Extension(user)| handle_get_record(state, id, user))
            .post(|State(state), Path(id), Extension(user), Json(update)| handle_update_record(state, id, user, update))
            .delete(|State(state), Path(id), Extension(user)| handle_delete_record(state, id, user))
        );

    let users_router = Router::new().route("/", put(create_user).get(get_users));
    let data_router = Router::new()
        .nest("/pupils", pupils_router)
        .nest("/users", users_router)
        .nest("/metrics", metrics_router)
        .nest("/records", records_router);
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any); // TODOSERVER this needs to only be the actual url (research this!!)

    Router::new().nest(
        "/api",
        Router::new()
            .nest("/data", data_router)
            .layer(from_fn_with_state(Arc::clone(&state), auth_service))
            .nest("/auth", auth_router)
            .layer(cors_layer),
    )
}
