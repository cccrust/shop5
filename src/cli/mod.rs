pub mod fmt;
pub mod user;
pub mod product;
pub mod cart;
pub mod order;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shop5", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 初始化資料庫
    Init,
    /// 使用者管理
    User(UserCmd),
    /// 商品管理
    Product(ProductCmd),
    /// 購物車管理
    Cart(CartCmd),
    /// 訂單管理
    Order(OrderCmd),
}

#[derive(Parser)]
pub struct UserCmd {
    #[command(subcommand)]
    pub subcommand: user::Subcommand,
}

#[derive(Parser)]
pub struct ProductCmd {
    #[command(subcommand)]
    pub subcommand: product::Subcommand,
}

#[derive(Parser)]
pub struct CartCmd {
    #[command(subcommand)]
    pub subcommand: cart::Subcommand,
}

#[derive(Parser)]
pub struct OrderCmd {
    #[command(subcommand)]
    pub subcommand: order::Subcommand,
}
