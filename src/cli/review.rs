use anyhow::Result;
use rusqlite::Connection;

use crate::model;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 新增評價
    Add {
        order_id: i64,
        user_id: i64,
        product_id: i64,
        rating: i64,
        #[arg(long, default_value = "")]
        content: String,
    },
    /// 檢視商品評價
    List { product_id: i64 },
    /// 檢視單筆評價
    Get { id: i64 },
    /// 刪除評價
    Delete { id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { order_id, user_id, product_id, rating, content } => {
            let r = model::review::add(conn, *order_id, *user_id, *product_id, *rating, content)?;
            println!("評價已新增：");
            super::fmt::fmt_review(conn, &r);
        }
        Subcommand::List { product_id } => {
            let reviews = model::review::list_by_product(conn, *product_id)?;
            if reviews.is_empty() {
                println!("此商品尚無評價");
            } else {
                println!("商品 #{} 的評價：", product_id);
                for r in &reviews {
                    super::fmt::fmt_review(conn, r);
                }
            }
        }
        Subcommand::Get { id } => {
            let r = model::review::get(conn, *id)?;
            super::fmt::fmt_review(conn, &r);
        }
        Subcommand::Delete { id } => {
            model::review::delete(conn, *id)?;
            println!("評價已刪除");
        }
    }
    Ok(())
}
