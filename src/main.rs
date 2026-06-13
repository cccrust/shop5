#![allow(dead_code, unused)]

mod cli;
mod db;
mod model;
// mod web; // v0.2 啟用

use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let path = std::env::var("SHOP5_DB").unwrap_or_else(|_| "shop5.db".to_string());
    PathBuf::from(path)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    match &cli.command {
        cli::Commands::Init => {
            db::init_db(&conn)?;
            println!("資料庫已初始化：{}", db_path.display());
        }
        cli::Commands::User(cmd) => {
            cli::user::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Product(cmd) => {
            cli::product::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Cart(cmd) => {
            cli::cart::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Order(cmd) => {
            cli::order::run(&conn, &cmd.subcommand)?;
        }
    }

    Ok(())
}
