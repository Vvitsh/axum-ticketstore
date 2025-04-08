mod health_check;
mod ticket;
mod ticket_get;
mod ticket_update_atomic;
mod user;
mod user_auth;

use health_check::health_check;
use ticket::ticket_create;
use ticket_get::{ticket_get_all, ticket_get_single};
use ticket_update_atomic::ticket_update_atomic;
use user::create_user;
use user_auth::{login, logout};

use axum::{
    Router,
    extract::FromRef,
    http::Method,
    middleware,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::guard::guard;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let app_state = AppState { db_conn };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            // Method::DELETE,
        ])
        .allow_origin(Any);

    // FIX: nesting routes
    // let user_routes = Router::new()
    // .route("/logout", post(logout))
    // .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
    // .route("/create_user", post(create_user))
    // .route("/login", post(login));

    // .nest("/users", user_routes)
    Router::new()
        // Auth routes
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        // Ticket routes
        .route("/tickets", post(ticket_create))
        .route("/tickets", get(ticket_get_all))
        .route("/tickets/{id}", get(ticket_get_single))
        .route("/tickets/{id}", put(ticket_update_atomic))
        // Health check routes
        .route("/health_check", get(health_check))
        .with_state(app_state)
        .layer(cors)
}
