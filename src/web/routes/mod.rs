pub mod cart;
pub mod order;
pub mod product;
pub mod user;
pub mod category;
pub mod seller;
pub mod review;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_routes() -> Router<crate::web::AppState> {
    Router::new()
        .route("/users", get(user::list).post(user::create))
        .route("/users/{id}", get(user::get).put(user::update).delete(user::delete))
        .route("/products", get(product::list).post(product::create))
        .route("/products/{id}", get(product::get).put(product::update).delete(product::delete))
        .route("/products/search", get(product::search))
        .route("/cart/{user_id}", get(cart::list).delete(cart::clear))
        .route("/cart", post(cart::add).delete(cart::remove))
        .route("/orders", get(order::list).post(order::create))
        .route("/orders/{id}", get(order::get).put(order::update))
        .route("/categories", get(category::list).post(category::create))
        .route("/categories/{id}", delete(category::delete))
        .route("/seller/{id}/orders", get(seller::orders))
        .route("/seller/{id}/products", get(seller::products))
        .merge(review::routes())
}
