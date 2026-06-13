pub mod cart;
pub mod order;
pub mod product;
pub mod user;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_routes() -> Router<crate::web::AppState> {
    Router::new()
        .route("/users", get(user::list).post(user::create))
        .route("/users/{id}", get(user::get).put(user::update).delete(user::delete))
        .route("/products", get(product::list).post(product::create))
        .route("/products/{id}", get(product::get).put(product::update).delete(product::delete))
        .route("/cart/{user_id}", get(cart::list).delete(cart::clear))
        .route("/cart", post(cart::add).delete(cart::remove))
        .route("/orders", get(order::list).post(order::create))
        .route("/orders/{id}", get(order::get).put(order::update))
}
