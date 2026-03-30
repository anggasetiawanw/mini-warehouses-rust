use axum::{ routing::{ get, post }, Router };

use crate::handlers::{ auth::{ login, AppState }, health::health_check };

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new().route("/health", get(health_check)).route("/auth/login", post(login))
        )
        .with_state(state)
}
