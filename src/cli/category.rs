use anyhow::Result;
use rusqlite::Connection;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 新增分類
    Add {
        name: String,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    /// 列出分類
    List,
    /// 檢視分類
    Get { id: i64 },
    /// 刪除分類
    Delete { id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { name, parent_id } => {
            let c = crate::model::category::add(conn, name, *parent_id)?;
            println!("已建立分類 #{}: {}", c.id, c.name);
        }
        Subcommand::List => {
            let categories = crate::model::category::list(conn)?;
            if categories.is_empty() {
                println!("查無分類");
                return Ok(());
            }
            println!("分類列表 ({} 項)", categories.len());
            for c in &categories {
                crate::cli::fmt::fmt_category(c);
            }
        }
        Subcommand::Get { id } => {
            let c = crate::model::category::get(conn, *id)?;
            crate::cli::fmt::fmt_category(&c);
        }
        Subcommand::Delete { id } => {
            crate::model::category::delete(conn, *id)?;
            println!("分類 #{} 已刪除", id);
        }
    }
    Ok(())
}
