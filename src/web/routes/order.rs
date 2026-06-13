use crate::model::order;
use crate::web::error::AppError;
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
