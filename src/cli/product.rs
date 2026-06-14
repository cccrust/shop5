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
        #[arg(long)]
        category_id: Option<i64>,
    },
    /// 列出商品
    List {
        #[arg(long)]
        seller_id: Option<i64>,
        #[arg(long, default_value = "all")]
        status: String,
        #[arg(long)]
        category_id: Option<i64>,
    },
    /// 搜尋商品
    Search {
        #[arg(long, default_value = "")]
        keyword: String,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        min_price: Option<i64>,
        #[arg(long)]
        max_price: Option<i64>,
        #[arg(long)]
        seller_id: Option<i64>,
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
        #[arg(long)]
        category_id: Option<i64>,
    },
    /// 刪除商品
    Delete { id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { seller_id, title, price, stock, desc, category_id } => {
            let p = crate::model::product::add(conn, *seller_id, title, *price, *stock, desc, *category_id)?;
            println!("已建立商品 #{}: {}", p.id, p.title);
        }
        Subcommand::List { seller_id, status, category_id } => {
            let products = crate::model::product::list(conn, *seller_id, status, *category_id)?;
            if products.is_empty() {
                println!("查無商品");
                return Ok(());
            }
            println!("商品列表 ({} 項)", products.len());
            for p in &products {
                crate::cli::fmt::fmt_product(p);
            }
        }
        Subcommand::Search { keyword, category_id, min_price, max_price, seller_id } => {
            let products = crate::model::product::search(conn, keyword, *category_id, *min_price, *max_price, *seller_id)?;
            if products.is_empty() {
                println!("查無符合條件的商品");
                return Ok(());
            }
            println!("搜尋結果 ({} 項)", products.len());
            for p in &products {
                crate::cli::fmt::fmt_product(p);
            }
        }
        Subcommand::Get { id } => {
            let p = crate::model::product::get(conn, *id)?;
            crate::cli::fmt::fmt_product(&p);
        }
        Subcommand::Update { id, title, price, stock, status, desc, category_id } => {
            let old = crate::model::product::get(conn, *id)?;
            let new_title = title.as_deref().unwrap_or(&old.title);
            let new_price = price.unwrap_or(old.price);
            let new_stock = stock.unwrap_or(old.stock);
            let new_status = status.as_deref().unwrap_or(&old.status);
            let new_desc = desc.as_deref().unwrap_or(&old.description);
            let new_cat = *category_id;
            let p = crate::model::product::update(conn, *id, new_title, new_price, new_stock, new_status, new_desc, new_cat)?;
            println!("商品 #{} 已更新", p.id);
        }
        Subcommand::Delete { id } => {
            crate::model::product::delete(conn, *id)?;
            println!("商品 #{} 已刪除", id);
        }
    }
    Ok(())
}
