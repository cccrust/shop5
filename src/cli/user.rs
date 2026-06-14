use anyhow::Result;
use rusqlite::Connection;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// 新增使用者（email 和 password 選填，CLI 不填則無法透過 Web 登入）
    Add {
        username: String,
        display_name: String,
        #[arg(long, default_value = "buyer")]
        role: String,
        #[arg(long, default_value = "")]
        bio: String,
        #[arg(long, default_value = "")]
        email: String,
        #[arg(long, default_value = "")]
        password: String,
    },
    /// 列出使用者
    List {
        #[arg(long, default_value = "")]
        search: String,
    },
    /// 檢視使用者
    Get { id: i64 },
    /// 更新使用者
    Update {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        bio: Option<String>,
        #[arg(long)]
        role: Option<String>,
    },
    /// 刪除使用者
    Delete { id: i64 },
}

pub fn run(conn: &Connection, cmd: &Subcommand) -> Result<()> {
    match cmd {
        Subcommand::Add { username, display_name, role, bio, email, password } => {
            let u = crate::model::user::add(conn, username, display_name, role, bio, email, password)?;
            println!("已建立使用者 #{}: @{} ({})", u.id, u.username, u.display_name);
        }
        Subcommand::List { search } => {
            let users = crate::model::user::list(conn, search)?;
            if users.is_empty() {
                println!("查無使用者");
                return Ok(());
            }
            println!("使用者列表 ({} 人)", users.len());
            for u in &users {
                crate::cli::fmt::fmt_user_brief(u);
            }
        }
        Subcommand::Get { id } => {
            let u = crate::model::user::get(conn, *id)?;
            crate::cli::fmt::fmt_user(conn, &u);
        }
        Subcommand::Update { id, name, bio, role } => {
            let old = crate::model::user::get(conn, *id)?;
            let new_name = name.as_deref().unwrap_or(&old.display_name);
            let new_bio = bio.as_deref().unwrap_or(&old.bio);
            let new_role = role.as_deref().unwrap_or(&old.role);
            let u = crate::model::user::update(conn, *id, new_name, new_bio, new_role)?;
            println!("使用者 #{} 已更新", u.id);
        }
        Subcommand::Delete { id } => {
            crate::model::user::delete(conn, *id)?;
            println!("使用者 #{} 已刪除", id);
        }
    }
    Ok(())
}
