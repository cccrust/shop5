use crate::model::user;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ListParams {
    search: Option<String>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    username: String,
    display_name: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    bio: String,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    display_name: Option<String>,
    bio: Option<String>,
    role: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<user::User>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let users = user::list(&conn, params.search.as_deref().unwrap_or(""))?;
    Ok(Json(users))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<user::User>, AppError> {
    let conn = state.conn.lock().unwrap();
    let u = user::get(&conn, id)?;
    Ok(Json(u))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<user::User>, AppError> {
    let conn = state.conn.lock().unwrap();
    let role = if payload.role.is_empty() { "buyer" } else { &payload.role };
    let u = user::add(&conn, &payload.username, &payload.display_name, role, &payload.bio)?;
    Ok(Json(u))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<user::User>, AppError> {
    let conn = state.conn.lock().unwrap();
    let old = user::get(&conn, id)?;
    let new_name = payload.display_name.as_deref().unwrap_or(&old.display_name);
    let new_bio = payload.bio.as_deref().unwrap_or(&old.bio);
    let new_role = payload.role.as_deref().unwrap_or(&old.role);
    let u = user::update(&conn, id, new_name, new_bio, new_role)?;
    Ok(Json(u))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    user::delete(&conn, id)?;
    Ok(Json(json!({ "deleted": true })))
}
