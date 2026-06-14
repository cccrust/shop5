use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

use crate::model;

use crate::web::AppState;

#[derive(serde::Deserialize)]
pub struct ReviewInput {
    order_id: i64,
    user_id: i64,
    product_id: i64,
    rating: i64,
    #[serde(default)]
    content: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reviews", post(create))
        .route("/reviews/{id}", get(get_review).delete(delete_review))
        .route("/reviews/product/{product_id}", get(list_by_product))
        .route("/reviews/user/{user_id}", get(list_by_user))
}

async fn create(
    State(state): State<AppState>,
    Json(input): Json<ReviewInput>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match model::review::add(
        &conn,
        input.order_id,
        input.user_id,
        input.product_id,
        input.rating,
        &input.content,
    ) {
        Ok(r) => (StatusCode::CREATED, Json(serde_json::json!(r))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn get_review(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match model::review::get(&conn, id) {
        Ok(r) => Json(serde_json::json!(r)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "評價不存在"}))).into_response(),
    }
}

async fn delete_review(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match model::review::delete(&conn, id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn list_by_product(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match model::review::list_by_product(&conn, product_id) {
        Ok(reviews) => Json(serde_json::json!(reviews)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn list_by_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match model::review::list_by_user(&conn, user_id) {
        Ok(reviews) => Json(serde_json::json!(reviews)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
