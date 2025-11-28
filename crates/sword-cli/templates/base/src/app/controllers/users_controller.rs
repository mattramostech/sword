use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::app::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, String)> {
    let user = state
        .user_service
        .create_user(payload.name, payload.email)
        .await
        .map_err(|e| {
            error!("Failed to create user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
        })?;

    let response = UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = state
        .user_service
        .get_user(id)
        .await
        .map_err(|e| {
            error!("Failed to get user {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
        })?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let response = UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
    };

    Ok(Json(response))
}
