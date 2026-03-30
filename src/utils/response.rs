use serde::Serialize;
use axum::{ http::StatusCode, Json };

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            data,
            message: "Success".to_string(),
        }
    }
    pub fn success_with_message(data: T, message: String) -> Self {
        ApiResponse {
            data,
            message,
        }
    }
}

impl ErrorResponse {
    pub fn error(message: String) -> Self {
        ErrorResponse {
            message,
            data: None,
        }
    }

    pub fn error_with_data(message: String, data: serde_json::Value) -> Self {
        ErrorResponse {
            message,
            data: Some(data),
        }
    }
}

pub fn success_response<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (StatusCode::OK, Json(ApiResponse::success(data)))
}

pub fn success_response_with_message<T: Serialize>(
    data: T,
    message: String
) -> (StatusCode, Json<ApiResponse<T>>) {
    (StatusCode::OK, Json(ApiResponse::success_with_message(data, message)))
}

pub fn error_response(message: String, status: StatusCode) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse::error(message)))
}
