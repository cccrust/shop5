use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Order {
    pub id: i64,
    pub buyer_id: i64,
    pub seller_id: i64,
    pub status: String,
    pub total: i64,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub product_title: String,
    pub product_price: i64,
    pub quantity: i64,
    pub subtotal: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CartPreview {
    pub items: Vec<OrderItem>,
    pub seller_id: i64,
    pub seller_name: String,
    pub total: i64,
    pub item_count: i64,
}

pub fn create_from_cart(conn: &Connection, buyer_id: i64, note: &str) -> Result<OrderWithItems> {
    let cart_items = crate::model::cart::list(conn, buyer_id)?;
    if cart_items.is_empty() {
        bail!("購物車是空的");
    }

    let mut seller_ids = Vec::new();
    for item in &cart_items {
        let p = crate::model::product::get(conn, item.product_id)?;
        if p.stock < item.quantity {
            bail!("商品「{}」庫存不足（庫存：{}，需求：{}）", p.title, p.stock, item.quantity);
        }
        if !seller_ids.contains(&p.seller_id) {
            seller_ids.push(p.seller_id);
        }
    }

    if seller_ids.len() > 1 {
        bail!("訂單包含多位賣家的商品，請分開下單");
    }

    let seller_id = seller_ids[0];
    let mut total = 0i64;
    let mut order_items = Vec::new();

    for item in &cart_items {
        let p = crate::model::product::get(conn, item.product_id)?;
        let subtotal = p.price * item.quantity;
        total += subtotal;
        order_items.push(OrderItem {
            id: 0,
            order_id: 0,
            product_id: p.id,
            product_title: p.title.clone(),
            product_price: p.price,
            quantity: item.quantity,
            subtotal,
        });
    }

    conn.execute(
        "INSERT INTO orders (buyer_id, seller_id, total, note) VALUES (?1, ?2, ?3, ?4)",
        params![buyer_id, seller_id, total, note],
    )?;
    let order_id = conn.last_insert_rowid();

    for oi in &order_items {
        conn.execute(
            "INSERT INTO order_items (order_id, product_id, product_title, product_price, quantity, subtotal) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![order_id, oi.product_id, oi.product_title, oi.product_price, oi.quantity, oi.subtotal],
        )?;
    }

    for item in &cart_items {
        conn.execute(
            "UPDATE products SET stock = stock - ?1, sales_count = sales_count + ?1 WHERE id = ?2",
            params![item.quantity, item.product_id],
        )?;
    }

    crate::model::cart::clear(conn, buyer_id)?;

    let order = get(conn, order_id)?;
    let items = get_items(conn, order_id)?;
    Ok(OrderWithItems { order, items })
}

pub fn preview_from_cart(conn: &Connection, buyer_id: i64) -> Result<CartPreview> {
    let cart_items = crate::model::cart::list(conn, buyer_id)?;
    if cart_items.is_empty() {
        bail!("購物車是空的");
    }

    let mut seller_ids = Vec::new();
    for item in &cart_items {
        let p = crate::model::product::get(conn, item.product_id)?;
        if p.stock < item.quantity {
            bail!("商品「{}」庫存不足（庫存：{}，需求：{}）", p.title, p.stock, item.quantity);
        }
        if !seller_ids.contains(&p.seller_id) {
            seller_ids.push(p.seller_id);
        }
    }

    if seller_ids.len() > 1 {
        bail!("購物車包含多位賣家的商品，請分開下單");
    }

    let seller_id = seller_ids[0];
    let seller_name: String = conn.query_row(
        "SELECT display_name FROM users WHERE id = ?1",
        params![seller_id],
        |row| row.get(0),
    )?;

    let mut total = 0i64;
    let mut items = Vec::new();
    for (idx, item) in cart_items.iter().enumerate() {
        let p = crate::model::product::get(conn, item.product_id)?;
        let subtotal = p.price * item.quantity;
        total += subtotal;
        items.push(OrderItem {
            id: idx as i64,
            order_id: 0,
            product_id: p.id,
            product_title: p.title.clone(),
            product_price: p.price,
            quantity: item.quantity,
            subtotal,
        });
    }

    Ok(CartPreview {
        item_count: cart_items.iter().map(|i| i.quantity).sum(),
        items,
        seller_id,
        seller_name,
        total,
    })
}

pub fn create_direct(
    conn: &Connection,
    buyer_id: i64,
    seller_id: i64,
    items: &[(i64, i64)], // [(product_id, quantity)]
    note: &str,
) -> Result<OrderWithItems> {
    if items.is_empty() {
        bail!("訂單項目不能為空");
    }

    let mut total = 0i64;
    let mut order_items = Vec::new();

    for &(product_id, quantity) in items {
        if quantity <= 0 {
            bail!("數量必須大於 0");
        }
        let p = crate::model::product::get(conn, product_id)?;
        if p.stock < quantity {
            bail!("商品「{}」庫存不足（庫存：{}，需求：{}）", p.title, p.stock, quantity);
        }
        let subtotal = p.price * quantity;
        total += subtotal;
        order_items.push(OrderItem {
            id: 0,
            order_id: 0,
            product_id: p.id,
            product_title: p.title.clone(),
            product_price: p.price,
            quantity,
            subtotal,
        });
    }

    conn.execute(
        "INSERT INTO orders (buyer_id, seller_id, total, note) VALUES (?1, ?2, ?3, ?4)",
        params![buyer_id, seller_id, total, note],
    )?;
    let order_id = conn.last_insert_rowid();

    for oi in &order_items {
        conn.execute(
            "INSERT INTO order_items (order_id, product_id, product_title, product_price, quantity, subtotal) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![order_id, oi.product_id, oi.product_title, oi.product_price, oi.quantity, oi.subtotal],
        )?;
    }

    for &(product_id, quantity) in items {
        conn.execute(
            "UPDATE products SET stock = stock - ?1, sales_count = sales_count + ?1 WHERE id = ?2",
            params![quantity, product_id],
        )?;
    }

    let order = get(conn, order_id)?;
    let items = get_items(conn, order_id)?;
    Ok(OrderWithItems { order, items })
}

pub fn get(conn: &Connection, id: i64) -> Result<Order> {
    let mut stmt = conn.prepare("SELECT * FROM orders WHERE id = ?1")?;
    let order = stmt.query_row(params![id], |row| {
        Ok(Order {
            id: row.get(0)?,
            buyer_id: row.get(1)?,
            seller_id: row.get(2)?,
            status: row.get(3)?,
            total: row.get(4)?,
            note: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    Ok(order)
}

pub fn get_items(conn: &Connection, order_id: i64) -> Result<Vec<OrderItem>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM order_items WHERE order_id = ?1 ORDER BY id",
    )?;
    let rows = stmt.query_map(params![order_id], |row| {
        Ok(OrderItem {
            id: row.get(0)?,
            order_id: row.get(1)?,
            product_id: row.get(2)?,
            product_title: row.get(3)?,
            product_price: row.get(4)?,
            quantity: row.get(5)?,
            subtotal: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn list(conn: &Connection, buyer_id: Option<i64>, seller_id: Option<i64>) -> Result<Vec<Order>> {
    let mut sql = String::from("SELECT * FROM orders WHERE 1=1");
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(bid) = buyer_id {
        sql.push_str(" AND buyer_id = ?");
        param_values.push(Box::new(bid));
    }
    if let Some(sid) = seller_id {
        sql.push_str(" AND seller_id = ?");
        param_values.push(Box::new(sid));
    }
    sql.push_str(" ORDER BY id DESC");
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Order {
            id: row.get(0)?,
            buyer_id: row.get(1)?,
            seller_id: row.get(2)?,
            status: row.get(3)?,
            total: row.get(4)?,
            note: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    let mut orders = Vec::new();
    for row in rows {
        orders.push(row?);
    }
    Ok(orders)
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<Order> {
    if !["pending", "paid", "shipped", "delivered", "cancelled"].contains(&status) {
        bail!("狀態必須為 pending、paid、shipped、delivered 或 cancelled");
    }
    let affected = conn.execute(
        "UPDATE orders SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![status, id],
    )?;
    if affected == 0 {
        bail!("訂單不存在");
    }
    get(conn, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::model::{user, product, cart};

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        user::add(&conn, "seller", "賣家", "seller", "", "", "").unwrap();
        user::add(&conn, "buyer", "買家", "buyer", "", "", "").unwrap();
        product::add(&conn, 1, "商品A", 100, 10, "", None).unwrap();
        product::add(&conn, 1, "商品B", 200, 5, "", None).unwrap();
        conn
    }

    #[test]
    fn test_create_order_from_cart() {
        let conn = setup();
        cart::add(&conn, 2, 1, 2).unwrap();
        cart::add(&conn, 2, 2, 1).unwrap();
        let result = create_from_cart(&conn, 2, "測試備註").unwrap();
        assert_eq!(result.order.status, "pending");
        assert_eq!(result.order.total, 400); // 100*2 + 200*1
        assert_eq!(result.order.note, "測試備註");
        assert_eq!(result.items.len(), 2);

        let updated = product::get(&conn, 1).unwrap();
        assert_eq!(updated.stock, 8);
        let updated2 = product::get(&conn, 2).unwrap();
        assert_eq!(updated2.stock, 4);

        let cart_items = cart::list(&conn, 2).unwrap();
        assert_eq!(cart_items.len(), 0);
    }

    #[test]
    fn test_create_order_empty_cart() {
        let conn = setup();
        let r = create_from_cart(&conn, 2, "");
        assert!(r.is_err());
    }

    #[test]
    fn test_create_direct_order() {
        let conn = setup();
        let items = vec![(1, 3), (2, 2)];
        let result = create_direct(&conn, 2, 1, &items, "直購").unwrap();
        assert_eq!(result.order.total, 700); // 100*3 + 200*2

        let updated = product::get(&conn, 1).unwrap();
        assert_eq!(updated.stock, 7);
    }

    #[test]
    fn test_list_orders() {
        let conn = setup();
        cart::add(&conn, 2, 1, 1).unwrap();
        create_from_cart(&conn, 2, "").unwrap();
        let orders = list(&conn, Some(2), None).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].buyer_id, 2);
    }

    #[test]
    fn test_update_status() {
        let conn = setup();
        cart::add(&conn, 2, 1, 1).unwrap();
        let result = create_from_cart(&conn, 2, "").unwrap();
        let updated = update_status(&conn, result.order.id, "paid").unwrap();
        assert_eq!(updated.status, "paid");
    }

    #[test]
    fn test_update_status_invalid() {
        let conn = setup();
        let r = update_status(&conn, 999, "paid");
        assert!(r.is_err());
    }
}
