use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, m),
            AppError::Internal(m) => {
                eprintln!("錯誤: {}", m);
                (StatusCode::INTERNAL_SERVER_ERROR, "伺服器內部錯誤".into())
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        let msg = e.to_string();
        if msg.contains("Query returned no rows") || msg.contains("不存在") {
            AppError::NotFound(msg)
        } else if msg.contains("不能為空") || msg.contains("不能為負數") || msg.contains("必須大於 0")
            || msg.contains("必須為") || msg.contains("已下架") || msg.contains("庫存不足")
            || msg.contains("購物車中無此商品") || msg.contains("購物車是空的")
            || msg.contains("請分開下單") || msg.contains("訂單項目不能為空")
        {
            AppError::BadRequest(msg)
        } else {
            eprintln!("伺服器錯誤: {}", msg);
            AppError::Internal("伺服器內部錯誤".into())
        }
    }
}
