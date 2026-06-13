use anyhow::Result;
use rusqlite::Connection;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 從購物車建立訂單
    Create {
        buyer_id: i64,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// 直購（跳過購物車直接下單）
    Buy {
        buyer_id: i64,
        seller_id: i64,
        product_id: i64,
        quantity: i64,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// 列出訂單
    List {
        #[arg(long)]
        buyer_id: Option<i64>,
        #[arg(long)]
        seller_id: Option<i64>,
    },
    /// 檢視訂單
    Get { id: i64 },
    /// 更新訂單狀態
    Update {
        id: i64,
        #[arg(long)]
        status: String,
    },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Create { buyer_id, note } => {
            let result = crate::model::order::create_from_cart(conn, *buyer_id, note)?;
            println!("已建立訂單 #{}", result.order.id);
            println!("  總金額: NT${}", result.order.total);
            println!("  項目數: {}", result.items.len());
        }
        Subcommand::Buy { buyer_id, seller_id, product_id, quantity, note } => {
            let items = vec![(*product_id, *quantity)];
            let result = crate::model::order::create_direct(conn, *buyer_id, *seller_id, &items, note)?;
            println!("已建立訂單 #{}", result.order.id);
            println!("  總金額: NT${}", result.order.total);
        }
        Subcommand::List { buyer_id, seller_id } => {
            let orders = crate::model::order::list(conn, *buyer_id, *seller_id)?;
            if orders.is_empty() {
                println!("查無訂單");
                return Ok(());
            }
            println!("訂單列表 ({} 筆)", orders.len());
            for o in &orders {
                crate::cli::fmt::fmt_order(conn, o);
            }
        }
        Subcommand::Get { id } => {
            let o = crate::model::order::get(conn, *id)?;
            let items = crate::model::order::get_items(conn, *id)?;
            crate::cli::fmt::fmt_order(conn, &o);
            println!("  明細:");
            for oi in &items {
                crate::cli::fmt::fmt_order_item(oi);
            }
        }
        Subcommand::Update { id, status } => {
            let o = crate::model::order::update_status(conn, *id, status)?;
            println!("訂單 #{} 狀態已更新為 {}", o.id, o.status);
        }
    }
    Ok(())
}
