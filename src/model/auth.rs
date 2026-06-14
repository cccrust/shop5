use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: super::user::User,
}

pub fn register(
    conn: &Connection,
    username: &str,
    display_name: &str,
    email: &str,
    password: &str,
    role: &str,
) -> Result<LoginResponse> {
    if username.is_empty() {
        bail!("使用者名稱不能為空");
    }
    if password.len() < 4 {
        bail!("密碼長度至少 4 碼");
    }
    if !["buyer", "seller", "admin"].contains(&role) {
        bail!("角色必須為 buyer、seller 或 admin");
    }
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    conn.execute(
        "INSERT INTO users (username, display_name, email, password_hash, role) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![username, display_name, email, hash, role],
    )?;
    let id = conn.last_insert_rowid();
    let user = super::user::get(conn, id)?;
    let token = new_token(conn, id)?;
    Ok(LoginResponse { token, user })
}

pub fn login(conn: &Connection, username: &str, password: &str) -> Result<LoginResponse> {
    let mut stmt = conn.prepare(
        "SELECT id, username, display_name, email, password_hash, role, bio, avatar, created_at, updated_at FROM users WHERE username = ?1"
    )?;
    let row = stmt.query_row(params![username], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, String>(9)?,
        ))
    }).map_err(|_| anyhow::anyhow!("帳號或密碼錯誤"))?;

    let (id, username, display_name, email, password_hash, role, bio, avatar, created_at, updated_at) = row;
    if password_hash.is_empty() || !bcrypt::verify(password, &password_hash)? {
        bail!("帳號或密碼錯誤");
    }
    let user = super::user::User { id, username, display_name, email, role, bio, avatar, created_at, updated_at };
    let token = new_token(conn, id)?;
    Ok(LoginResponse { token, user })
}

pub fn logout(conn: &Connection, token: &str) -> Result<()> {
    conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
    Ok(())
}

pub fn get_user_by_token(conn: &Connection, token: &str) -> Result<super::user::User> {
    let mut stmt = conn.prepare(
        "SELECT u.id, u.username, u.display_name, u.email, u.password_hash, u.role, u.bio, u.avatar, u.created_at, u.updated_at
         FROM sessions s JOIN users u ON s.user_id = u.id
         WHERE s.token = ?1 AND s.expires_at > datetime('now')"
    )?;
    let row = stmt.query_row(params![token], |row| {
        Ok(super::user::User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            email: row.get(3)?,
            role: row.get(5)?,
            bio: row.get(6)?,
            avatar: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|_| anyhow::anyhow!("未授權"))?;
    Ok(row)
}

fn new_token(conn: &Connection, user_id: i64) -> Result<String> {
    let token = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO sessions (user_id, token, expires_at) VALUES (?1, ?2, datetime('now', '+7 days'))",
        params![user_id, token],
    )?;
    Ok(token)
}
