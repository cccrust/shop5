use crate::model::cart;
use crate::web::error::AppError;
use crate::web::middleware::AuthUser;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct AddPayload {
    user_id: i64,
    product_id: i64,
    #[serde(default = "default_qty")]
    quantity: i64,
}

fn default_qty() -> i64 { 1 }

#[derive(Deserialize)]
pub struct RemovePayload {
    user_id: i64,
    product_id: i64,
}

#[derive(Deserialize)]
pub struct UpdateQtyPayload {
    quantity: i64,
}

// ---- 舊端點（需傳 user_id），保留向後相容 ----

pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<cart::CartItemWithProduct>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = cart::list(&conn, user_id)?;
    Ok(Json(items))
}

pub async fn add(
    State(state): State<AppState>,
    Json(payload): Json<AddPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::add(&conn, payload.user_id, payload.product_id, payload.quantity)?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn remove(
    State(state): State<AppState>,
    Json(payload): Json<RemovePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::remove(&conn, payload.user_id, payload.product_id)?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn update_qty(
    State(state): State<AppState>,
    Path((user_id, product_id)): Path<(i64, i64)>,
    Json(payload): Json<UpdateQtyPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::update_qty(&conn, user_id, product_id, payload.quantity)?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn clear(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::clear(&conn, user_id)?;
    Ok(Json(json!({ "ok": true })))
}

// ---- 新端點：從 token 推斷 user_id ----

pub async fn my_list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<cart::CartItemWithProduct>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = cart::list(&conn, auth.0.id)?;
    Ok(Json(items))
}

pub async fn my_clear(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::clear(&conn, auth.0.id)?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn my_update_qty(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<i64>,
    Json(payload): Json<UpdateQtyPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::update_qty(&conn, auth.0.id, product_id, payload.quantity)?;
    Ok(Json(json!({ "ok": true })))
}
