use anyhow::Result;
use rusqlite::Connection;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 新增商品
    Add {
        seller_id: i64,
        title: String,
        price: i64,
        stock: i64,
        #[arg(long, default_value = "")]
        desc: String,
    },
    /// 列出商品
    List {
        #[arg(long)]
        seller_id: Option<i64>,
        #[arg(long, default_value = "all")]
        status: String,
    },
    /// 檢視商品
    Get { id: i64 },
    /// 更新商品
    Update {
        id: i64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        price: Option<i64>,
        #[arg(long)]
        stock: Option<i64>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        desc: Option<String>,
    },
    /// 刪除商品
    Delete { id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { seller_id, title, price, stock, desc } => {
            let p = crate::model::product::add(conn, *seller_id, title, *price, *stock, desc)?;
            println!("已建立商品 #{}: {}", p.id, p.title);
        }
        Subcommand::List { seller_id, status } => {
            let products = crate::model::product::list(conn, *seller_id, status)?;
            if products.is_empty() {
                println!("查無商品");
                return Ok(());
            }
            println!("商品列表 ({} 項)", products.len());
            for p in &products {
                crate::cli::fmt::fmt_product(p);
            }
        }
        Subcommand::Get { id } => {
            let p = crate::model::product::get(conn, *id)?;
            crate::cli::fmt::fmt_product(&p);
        }
        Subcommand::Update { id, title, price, stock, status, desc } => {
            let old = crate::model::product::get(conn, *id)?;
            let new_title = title.as_deref().unwrap_or(&old.title);
            let new_price = price.unwrap_or(old.price);
            let new_stock = stock.unwrap_or(old.stock);
            let new_status = status.as_deref().unwrap_or(&old.status);
            let new_desc = desc.as_deref().unwrap_or(&old.description);
            let p = crate::model::product::update(conn, *id, new_title, new_price, new_stock, new_status, new_desc)?;
            println!("商品 #{} 已更新", p.id);
        }
        Subcommand::Delete { id } => {
            crate::model::product::delete(conn, *id)?;
            println!("商品 #{} 已刪除", id);
        }
    }
    Ok(())
}
