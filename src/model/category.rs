use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}

pub fn add(conn: &Connection, name: &str, parent_id: Option<i64>) -> Result<Category> {
    let name = name.trim();
    if name.is_empty() {
        bail!("分類名稱不能為空");
    }
    conn.execute(
        "INSERT INTO categories (name, parent_id) VALUES (?1, ?2)",
        params![name, parent_id],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)
}

pub fn list(conn: &Connection) -> Result<Vec<Category>> {
    let mut stmt = conn.prepare("SELECT * FROM categories ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;
    let mut categories = Vec::new();
    for row in rows {
        categories.push(row?);
    }
    Ok(categories)
}

pub fn get(conn: &Connection, id: i64) -> Result<Category> {
    let mut stmt = conn.prepare("SELECT * FROM categories WHERE id = ?1")?;
    let cat = stmt.query_row(params![id], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;
    Ok(cat)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;
    if affected == 0 {
        bail!("分類不存在");
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
    fn test_add_and_get_category() {
        let conn = setup();
        let c = add(&conn, "3C", None).unwrap();
        assert_eq!(c.name, "3C");
        assert!(c.parent_id.is_none());
        let got = get(&conn, c.id).unwrap();
        assert_eq!(got.name, "3C");
    }

    #[test]
    fn test_add_subcategory() {
        let conn = setup();
        let parent = add(&conn, "3C", None).unwrap();
        let child = add(&conn, "手機", Some(parent.id)).unwrap();
        assert_eq!(child.parent_id, Some(parent.id));
    }

    #[test]
    fn test_list_categories() {
        let conn = setup();
        add(&conn, "3C", None).unwrap();
        add(&conn, "服飾", None).unwrap();
        assert_eq!(list(&conn).unwrap().len(), 2);
    }

    #[test]
    fn test_delete_category() {
        let conn = setup();
        let c = add(&conn, "3C", None).unwrap();
        delete(&conn, c.id).unwrap();
        assert!(get(&conn, c.id).is_err());
    }
}
