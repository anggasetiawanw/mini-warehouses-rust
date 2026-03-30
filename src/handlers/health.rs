use axum::{ extract::Json };
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

pub async fn health_check() -> (
    axum::http::StatusCode,
    Json<crate::utils::response::ApiResponse<HealthResponse>>,
) {
    let response = HealthResponse {
        status: "OK".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    crate::utils::response::success_response(response)
}
