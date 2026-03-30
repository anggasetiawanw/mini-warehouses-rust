use axum::{ extract::State, Json };
use bcrypt::verify;
use serde::{ Deserialize, Serialize };
use tracing::{ debug, error, warn };

use crate::configs::{ db::DBPool, security::{ JwtConfig, sign_jwt } };
use crate::repositories::user_repository::find_user_with_role_by_email;
use crate::utils::response::{ success_response_with_message };

#[derive(Clone)]
pub struct AppState {
    pub pool: DBPool,
    pub jwt: JwtConfig,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
    pub roles: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<
    (axum::http::StatusCode, Json<crate::utils::response::ApiResponse<LoginResponse>>),
    (axum::http::StatusCode, Json<crate::utils::response::ErrorResponse>)
> {
    let found = match find_user_with_role_by_email(&state.pool, &payload.email).await {
        Ok(result) => result,
        Err(err) => {
            error!(error = ?err, email = %payload.email, "failed to find user by email");
            return Err(
                crate::utils::response::error_response(
                    "Internal server error".to_string(),
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                )
            );
        }
    };

    let (user, role) = match found {
        Some(tuple) => tuple,
        None => {
            warn!("login failed: user not found for email={}", payload.email);
            return Err(
                crate::utils::response::error_response(
                    "Invalid email or password".to_string(),
                    axum::http::StatusCode::UNAUTHORIZED
                )
            );
        }
    };

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        warn!("login failed: invalid password for user_id={}", user.id);
        return Err(
            crate::utils::response::error_response(
                "Invalid email or password".to_string(),
                axum::http::StatusCode::UNAUTHORIZED
            )
        );
    }

    let role_name = role.unwrap_or_else(|| "Keeper".to_string());

    let token = sign_jwt(&state.jwt, user.id, &user.name, &user.email, &role_name).map_err(|err| {
        error!(error = ?err, user_id = user.id, "failed to sign jwt");
        crate::utils::response::error_response(
            "Failed to generate token".to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        )
    })?;

    let user_info = UserInfo {
        id: user.id,
        email: user.email,
        roles: role_name,
    };

    let response = LoginResponse {
        token,
        user: user_info,
    };

    Ok(success_response_with_message(response, "Login successful".to_string()))
}
