pub mod auth;
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
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))
        .route("/users", get(user::list).post(user::create))
        .route("/users/{id}", get(user::get).put(user::update).delete(user::delete))
        .route("/products", get(product::list).post(product::create))
        .route("/products/{id}", get(product::get).put(product::update).delete(product::delete))
        .route("/products/search", get(product::search))
        .route("/cart", post(cart::add).delete(cart::remove))
        .route("/cart/{user_id}", get(cart::list).delete(cart::clear))
        .route("/cart/{user_id}/{product_id}", put(cart::update_qty))
        .route("/cart/me", get(cart::my_list).delete(cart::my_clear))
        .route("/cart/me/{product_id}", put(cart::my_update_qty))
        .route("/orders", get(order::list).post(order::create))
        .route("/orders/preview", post(order::preview))
        .route("/orders/{id}", get(order::get).put(order::update))
        .route("/orders/me", get(order::my_list).post(order::my_create))
        .route("/orders/me/preview", post(order::my_preview))
        .route("/categories", get(category::list).post(category::create))
        .route("/categories/{id}", delete(category::delete))
        .route("/seller/{id}/orders", get(seller::orders))
        .route("/seller/{id}/products", get(seller::products))
        .route("/seller/{id}/stats", get(seller::stats))
        .route("/seller/me/orders", get(seller::my_orders))
        .route("/seller/me/products", get(seller::my_products))
        .route("/seller/me/stats", get(seller::my_stats))
        .merge(review::routes())
}
