use crate::model::category;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreatePayload {
    name: String,
    parent_id: Option<i64>,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<category::Category>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let categories = category::list(&conn)?;
    Ok(Json(categories))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<category::Category>, AppError> {
    let conn = state.conn.lock().unwrap();
    let c = category::add(&conn, &payload.name, payload.parent_id)?;
    Ok(Json(c))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    category::delete(&conn, id)?;
    Ok(Json(json!({ "deleted": true })))
}
