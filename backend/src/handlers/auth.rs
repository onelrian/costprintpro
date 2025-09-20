use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    Extension,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{LoginRequest, LoginResponse, UserInfo, UserRole},
    services::user_service::UserService,
    utils::errors::AppError,
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    use crate::models::{UserRole, UserInfo};
    
    // For demo purposes, accept any email/password combination
    // In production, you would verify against the database
    let user_info = UserInfo {
        id: Uuid::new_v4(),
        email: payload.email.clone(),
        name: "Demo User".to_string(),
        role: UserRole::User,
    };
    
    // Generate JWT token
    let claims = Claims {
        sub: user_info.id.to_string(),
        email: user_info.email.clone(),
        role: user_info.role.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    ).map_err(|_| AppError::InternalServerError("Failed to generate token".to_string()))?;
    
    Ok(Json(LoginResponse {
        token,
        user: user_info,
    }))
}

pub async fn logout() -> Result<Json<serde_json::Value>, StatusCode> {
    // Since we're using stateless JWT tokens, logout is handled client-side
    // by removing the token from storage
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

pub async fn me(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<UserInfo>, AppError> {
    use crate::models::{UserRole, UserInfo};
    
    // Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = auth_header.ok_or(AppError::Unauthorized("Missing token".to_string()))?;

    // Decode and validate token
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let user_info = UserInfo {
        id: Uuid::parse_str(&claims.claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?,
        email: claims.claims.email,
        name: "Demo User".to_string(),
        role: UserRole::User,
    };
    
    Ok(Json(user_info))
}

// Middleware to extract user from JWT token
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = auth_header.ok_or(AppError::Unauthorized("Missing token".to_string()))?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let user_id = Uuid::parse_str(&claims.claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    let user_service = UserService::new(&state.db.connection);
    let user = user_service
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    if !user.is_active {
        return Err(AppError::Unauthorized("Account is disabled".to_string()));
    }

    let user_info = UserInfo::from(user);
    request.extensions_mut().insert(user_info);

    Ok(next.run(request).await)
}
