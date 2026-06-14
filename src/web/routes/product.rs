use crate::model::product;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ListParams {
    seller_id: Option<i64>,
    status: Option<String>,
    category_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    q: Option<String>,
    category_id: Option<i64>,
    min_price: Option<i64>,
    max_price: Option<i64>,
    seller_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    seller_id: i64,
    title: String,
    price: i64,
    stock: i64,
    #[serde(default)]
    description: String,
    category_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    title: Option<String>,
    price: Option<i64>,
    stock: Option<i64>,
    status: Option<String>,
    description: Option<String>,
    category_id: Option<i64>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let status = params.status.as_deref().unwrap_or("all");
    let products = product::list(&conn, params.seller_id, status, params.category_id)?;
    Ok(Json(products))
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let keyword = params.q.as_deref().unwrap_or("");
    let products = product::search(&conn, keyword, params.category_id, params.min_price, params.max_price, params.seller_id)?;
    Ok(Json(products))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = product::get(&conn, id)?;
    Ok(Json(p))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = product::add(&conn, payload.seller_id, &payload.title, payload.price, payload.stock, &payload.description, payload.category_id)?;
    Ok(Json(p))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let old = product::get(&conn, id)?;
    let new_title = payload.title.as_deref().unwrap_or(&old.title);
    let new_price = payload.price.unwrap_or(old.price);
    let new_stock = payload.stock.unwrap_or(old.stock);
    let new_status = payload.status.as_deref().unwrap_or(&old.status);
    let new_desc = payload.description.as_deref().unwrap_or(&old.description);
    let new_cat = payload.category_id;
    let p = product::update(&conn, id, new_title, new_price, new_stock, new_status, new_desc, new_cat)?;
    Ok(Json(p))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    product::delete(&conn, id)?;
    Ok(Json(json!({ "deleted": true })))
}
