use crate::model::auth;
use crate::web::error::AppError;
use crate::web::middleware::AuthUser;
use crate::web::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    display_name: String,
    #[serde(default)]
    email: String,
    password: String,
    #[serde(default = "default_role")]
    role: String,
}

fn default_role() -> String { "buyer".into() }

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<auth::LoginResponse>, AppError> {
    let conn = state.conn.lock().unwrap();
    let result = auth::register(&conn, &payload.username, &payload.display_name, &payload.email, &payload.password, &payload.role)?;
    Ok(Json(result))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<auth::LoginResponse>, AppError> {
    let conn = state.conn.lock().unwrap();
    let result = auth::login(&conn, &payload.username, &payload.password)
        .map_err(|_| AppError::Unauthorized("帳號或密碼錯誤".into()))?;
    Ok(Json(result))
}

pub async fn logout(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    conn.execute("DELETE FROM sessions WHERE user_id = ?1", rusqlite::params![auth.0.id])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

pub async fn me(
    auth: AuthUser,
) -> Result<Json<crate::model::user::User>, AppError> {
    Ok(Json(auth.0))
}
