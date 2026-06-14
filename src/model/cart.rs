use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct CartItem {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CartItemWithProduct {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub title: String,
    pub price: i64,
    pub stock: i64,
}

pub fn add(conn: &Connection, user_id: i64, product_id: i64, quantity: i64) -> Result<CartItem> {
    if quantity <= 0 {
        bail!("數量必須大於 0");
    }
    let product = crate::model::product::get(conn, product_id)?;
    if product.status != "active" {
        bail!("商品已下架");
    }
    if product.stock < 1 {
        bail!("商品庫存不足");
    }
    conn.execute(
        "INSERT INTO cart_items (user_id, product_id, quantity) VALUES (?1, ?2, ?3)
         ON CONFLICT(user_id, product_id) DO UPDATE SET quantity = quantity + ?3",
        params![user_id, product_id, quantity],
    )?;
    get_by_user_product(conn, user_id, product_id)
}

fn get_by_user_product(conn: &Connection, user_id: i64, product_id: i64) -> Result<CartItem> {
    let mut stmt = conn.prepare("SELECT * FROM cart_items WHERE user_id = ?1 AND product_id = ?2")?;
    let item = stmt.query_row(params![user_id, product_id], |row| {
        Ok(CartItem {
            id: row.get(0)?,
            user_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
        })
    })?;
    Ok(item)
}

pub fn remove(conn: &Connection, user_id: i64, product_id: i64) -> Result<()> {
    let affected = conn.execute(
        "DELETE FROM cart_items WHERE user_id = ?1 AND product_id = ?2",
        params![user_id, product_id],
    )?;
    if affected == 0 {
        bail!("購物車中無此商品");
    }
    Ok(())
}

pub fn list(conn: &Connection, user_id: i64) -> Result<Vec<CartItemWithProduct>> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.user_id, c.product_id, c.quantity, p.title, p.price, p.stock
         FROM cart_items c
         JOIN products p ON p.id = c.product_id
         WHERE c.user_id = ?1
         ORDER BY c.id",
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(CartItemWithProduct {
            id: row.get(0)?,
            user_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
            title: row.get(4)?,
            price: row.get(5)?,
            stock: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn update_qty(conn: &Connection, user_id: i64, product_id: i64, quantity: i64) -> Result<CartItem> {
    if quantity <= 0 {
        bail!("數量必須大於 0");
    }
    let product = crate::model::product::get(conn, product_id)?;
    if product.status != "active" {
        bail!("商品已下架");
    }
    if product.stock < quantity {
        bail!("商品庫存不足（庫存：{}，需求：{}）", product.stock, quantity);
    }
    let affected = conn.execute(
        "UPDATE cart_items SET quantity = ?1 WHERE user_id = ?2 AND product_id = ?3",
        params![quantity, user_id, product_id],
    )?;
    if affected == 0 {
        bail!("購物車中無此商品");
    }
    get_by_user_product(conn, user_id, product_id)
}

pub fn clear(conn: &Connection, user_id: i64) -> Result<()> {
    conn.execute("DELETE FROM cart_items WHERE user_id = ?1", params![user_id])?;
    Ok(())
}

pub fn count(conn: &Connection, user_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COALESCE(SUM(quantity), 0) FROM cart_items WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::model::{user, product};

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        user::add(&conn, "seller", "賣家", "seller", "", "", "").unwrap();
        user::add(&conn, "buyer", "買家", "buyer", "", "", "").unwrap();
        product::add(&conn, 1, "商品A", 100, 10, "", None).unwrap();
        conn
    }

    #[test]
    fn test_add_to_cart() {
        let conn = setup();
        let item = add(&conn, 2, 1, 2).unwrap();
        assert_eq!(item.quantity, 2);
    }

    #[test]
    fn test_add_duplicate_increases_quantity() {
        let conn = setup();
        add(&conn, 2, 1, 2).unwrap();
        add(&conn, 2, 1, 3).unwrap();
        let item = get_by_user_product(&conn, 2, 1).unwrap();
        assert_eq!(item.quantity, 5);
    }

    #[test]
    fn test_add_invalid_quantity() {
        let conn = setup();
        let r = add(&conn, 2, 1, 0);
        assert!(r.is_err());
    }

    #[test]
    fn test_remove_from_cart() {
        let conn = setup();
        add(&conn, 2, 1, 1).unwrap();
        remove(&conn, 2, 1).unwrap();
        let items = list(&conn, 2).unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_list_cart() {
        let conn = setup();
        add(&conn, 2, 1, 3).unwrap();
        let items = list(&conn, 2).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "商品A");
        assert_eq!(items[0].quantity, 3);
    }

    #[test]
    fn test_clear_cart() {
        let conn = setup();
        add(&conn, 2, 1, 1).unwrap();
        clear(&conn, 2).unwrap();
        let items = list(&conn, 2).unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_count() {
        let conn = setup();
        assert_eq!(count(&conn, 2).unwrap(), 0);
        add(&conn, 2, 1, 2).unwrap();
        assert_eq!(count(&conn, 2).unwrap(), 2);
    }
}
