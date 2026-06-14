use crate::model::{order, product, stats};
use crate::web::error::AppError;
use crate::web::middleware::AuthUser;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;

// ---- 舊端點（需傳 seller_id 在 URL），保留向後相容 ----

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

// ---- 新端點：從 token 推斷 seller_id ----

pub async fn my_orders(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<order::Order>>, AppError> {
    let conn = state.conn.lock().unwrap();
    if auth.0.role != "seller" {
        return Err(AppError::Forbidden("只有賣家可以查看訂單".into()));
    }
    let orders = order::list(&conn, None, Some(auth.0.id))?;
    Ok(Json(orders))
}

pub async fn my_products(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    if auth.0.role != "seller" {
        return Err(AppError::Forbidden("只有賣家可以管理商品".into()));
    }
    let products = product::list(&conn, Some(auth.0.id), "all", None)?;
    Ok(Json(products))
}

pub async fn my_stats(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<stats::SellerStats>, AppError> {
    let conn = state.conn.lock().unwrap();
    if auth.0.role != "seller" {
        return Err(AppError::Forbidden("只有賣家可以查看儀表板".into()));
    }
    let s = stats::seller_stats(&conn, auth.0.id)?;
    Ok(Json(s))
}
