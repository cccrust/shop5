use anyhow::Result;
use rusqlite::Connection;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 加入購物車
    Add {
        user_id: i64,
        product_id: i64,
        #[arg(long, default_value = "1")]
        quantity: i64,
    },
    /// 移除購物車商品
    Remove { user_id: i64, product_id: i64 },
    /// 列出購物車
    List { user_id: i64 },
    /// 清空購物車
    Clear { user_id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { user_id, product_id, quantity } => {
            let item = crate::model::cart::add(conn, *user_id, *product_id, *quantity)?;
            println!("已加入購物車：商品 #{} x{}", item.product_id, item.quantity);
        }
        Subcommand::Remove { user_id, product_id } => {
            crate::model::cart::remove(conn, *user_id, *product_id)?;
            println!("已從購物車移除商品 #{}", product_id);
        }
        Subcommand::List { user_id } => {
            let items = crate::model::cart::list(conn, *user_id)?;
            if items.is_empty() {
                println!("購物車是空的");
                return Ok(());
            }
            let total: i64 = items.iter().map(|i| i.price * i.quantity).sum();
            println!("購物車 ({})", crate::cli::cart::fmt_count(conn, *user_id));
            for item in &items {
                crate::cli::fmt::fmt_cart_item(item);
            }
            println!("合計: NT${}", total);
        }
        Subcommand::Clear { user_id } => {
            crate::model::cart::clear(conn, *user_id)?;
            println!("購物車已清空");
        }
    }
    Ok(())
}

fn fmt_count(conn: &Connection, user_id: i64) -> String {
    match crate::model::cart::count(conn, user_id) {
        Ok(c) => format!("{} 項商品", c),
        Err(_) => String::new(),
    }
}
