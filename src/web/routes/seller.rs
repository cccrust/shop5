use crate::model::{order, product, stats};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;

pub async fn orders(
    State(state): State<AppState>,
    Path(seller_id): Path<i64>,
) -> Result<Json<Vec<order::Order>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let orders = order::list(&conn, None, Some(seller_id))?;
    Ok(Json(orders))
}

pub async fn products(
    State(state): State<AppState>,
    Path(seller_id): Path<i64>,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let products = product::list(&conn, Some(seller_id), "all", None)?;
    Ok(Json(products))
}

pub async fn stats(
    State(state): State<AppState>,
    Path(seller_id): Path<i64>,
) -> Result<Json<stats::SellerStats>, AppError> {
    let conn = state.conn.lock().unwrap();
    let s = stats::seller_stats(&conn, seller_id)?;
    Ok(Json(s))
}
