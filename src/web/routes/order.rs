use crate::model::order;
use crate::web::error::AppError;
use crate::web::middleware::AuthUser;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ListParams {
    buyer_id: Option<i64>,
    seller_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    buyer_id: i64,
    #[serde(default)]
    note: String,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    status: String,
}

#[derive(Deserialize)]
pub struct PreviewPayload {
    buyer_id: i64,
}

// ---- 舊端點（需傳 user_id），保留向後相容 ----

pub async fn preview(
    State(state): State<AppState>,
    Json(payload): Json<PreviewPayload>,
) -> Result<Json<order::CartPreview>, AppError> {
    let conn = state.conn.lock().unwrap();
    let preview = order::preview_from_cart(&conn, payload.buyer_id)?;
    Ok(Json(preview))
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<order::Order>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let orders = order::list(&conn, params.buyer_id, params.seller_id)?;
    Ok(Json(orders))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<order::OrderWithItems>, AppError> {
    let conn = state.conn.lock().unwrap();
    let o = order::get(&conn, id)?;
    let items = order::get_items(&conn, id)?;
    Ok(Json(order::OrderWithItems { order: o, items }))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<order::OrderWithItems>, AppError> {
    let conn = state.conn.lock().unwrap();
    let result = order::create_from_cart(&conn, payload.buyer_id, &payload.note)?;
    Ok(Json(result))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<order::Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    let o = order::update_status(&conn, id, &payload.status)?;
    Ok(Json(o))
}

// ---- 新端點：從 token 推斷 buyer_id ----

pub async fn my_list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<order::Order>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let orders = order::list(&conn, Some(auth.0.id), None)?;
    Ok(Json(orders))
}

pub async fn my_preview(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<order::CartPreview>, AppError> {
    let conn = state.conn.lock().unwrap();
    let preview = order::preview_from_cart(&conn, auth.0.id)?;
    Ok(Json(preview))
}

#[derive(Deserialize)]
pub struct MyCreatePayload {
    #[serde(default)]
    note: String,
}

pub async fn my_create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<MyCreatePayload>,
) -> Result<Json<order::OrderWithItems>, AppError> {
    let conn = state.conn.lock().unwrap();
    let result = order::create_from_cart(&conn, auth.0.id, &payload.note)?;
    Ok(Json(result))
}
