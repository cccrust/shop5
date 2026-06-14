use crate::web::error::AppError;
use crate::web::AppState;
use crate::model::user::User;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::sync::{Arc, Mutex};

pub struct AuthUser(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({ "error": "請先登入" })),
                )
            })?;

        let conn = state.conn.lock().unwrap();
        match crate::model::auth::get_user_by_token(&conn, token) {
            Ok(user) => Ok(AuthUser(user)),
            Err(_) => Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": "請先登入" })),
            )),
        }
    }
}
