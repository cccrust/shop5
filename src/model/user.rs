use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub bio: String,
    pub avatar: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn add(conn: &Connection, username: &str, display_name: &str, role: &str, bio: &str) -> Result<User> {
    if !["buyer", "seller", "admin"].contains(&role) {
        bail!("角色必須為 buyer、seller 或 admin");
    }
    conn.execute(
        "INSERT INTO users (username, display_name, role, bio) VALUES (?1, ?2, ?3, ?4)",
        params![username, display_name, role, bio],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)
}

pub fn list(conn: &Connection, search: &str) -> Result<Vec<User>> {
    let sql = if search.is_empty() {
        "SELECT * FROM users ORDER BY id".to_string()
    } else {
        format!(
            "SELECT * FROM users WHERE username LIKE '%{}%' OR display_name LIKE '%{}%' ORDER BY id",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        )
    };
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            role: row.get(3)?,
            bio: row.get(4)?,
            avatar: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }
    Ok(users)
}

pub fn get(conn: &Connection, id: i64) -> Result<User> {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?1")?;
    let user = stmt.query_row(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            role: row.get(3)?,
            bio: row.get(4)?,
            avatar: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    Ok(user)
}

pub fn update(conn: &Connection, id: i64, display_name: &str, bio: &str, role: &str) -> Result<User> {
    if !["buyer", "seller", "admin"].contains(&role) {
        bail!("角色必須為 buyer、seller 或 admin");
    }
    let affected = conn.execute(
        "UPDATE users SET display_name = ?1, bio = ?2, role = ?3, updated_at = datetime('now') WHERE id = ?4",
        params![display_name, bio, role, id],
    )?;
    if affected == 0 {
        bail!("使用者不存在");
    }
    get(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    if affected == 0 {
        bail!("使用者不存在");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        conn
    }

    #[test]
    fn test_add_and_get_user() {
        let conn = setup();
        let u = add(&conn, "alice", "愛麗絲", "buyer", "哈囉").unwrap();
        assert_eq!(u.username, "alice");
        assert_eq!(u.display_name, "愛麗絲");
        assert_eq!(u.role, "buyer");

        let got = get(&conn, u.id).unwrap();
        assert_eq!(got.username, "alice");
    }

    #[test]
    fn test_add_user_invalid_role() {
        let conn = setup();
        let r = add(&conn, "bad", "壞角色", "invalid", "");
        assert!(r.is_err());
    }

    #[test]
    fn test_list_users() {
        let conn = setup();
        add(&conn, "alice", "愛麗絲", "buyer", "").unwrap();
        add(&conn, "bob", "鮑勃", "seller", "").unwrap();
        let users = list(&conn, "").unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_list_users_search() {
        let conn = setup();
        add(&conn, "alice", "愛麗絲", "buyer", "").unwrap();
        add(&conn, "bob", "鮑勃", "seller", "").unwrap();
        let users = list(&conn, "愛麗").unwrap();
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_update_user() {
        let conn = setup();
        let u = add(&conn, "alice", "愛麗絲", "buyer", "哈囉").unwrap();
        update(&conn, u.id, "愛麗絲醬", "バイバイ", "seller").unwrap();
        let got = get(&conn, u.id).unwrap();
        assert_eq!(got.display_name, "愛麗絲醬");
        assert_eq!(got.role, "seller");
    }

    #[test]
    fn test_delete_user() {
        let conn = setup();
        let u = add(&conn, "alice", "愛麗絲", "buyer", "").unwrap();
        delete(&conn, u.id).unwrap();
        let r = get(&conn, u.id);
        assert!(r.is_err());
    }
}
