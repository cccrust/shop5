use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Product {
    pub id: i64,
    pub seller_id: i64,
    pub title: String,
    pub description: String,
    pub price: i64,
    pub stock: i64,
    pub category_id: Option<i64>,
    pub status: String,
    pub sales_count: i64,
    pub rating: f64,
    pub review_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub fn add(
    conn: &Connection,
    seller_id: i64,
    title: &str,
    price: i64,
    stock: i64,
    description: &str,
    category_id: Option<i64>,
) -> Result<Product> {
    if title.trim().is_empty() {
        bail!("商品名稱不能為空");
    }
    if price < 0 {
        bail!("價格不能為負數");
    }
    if stock < 0 {
        bail!("庫存不能為負數");
    }
    conn.execute(
        "INSERT INTO products (seller_id, title, price, stock, description, category_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![seller_id, title.trim(), price, stock, description, category_id],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)
}

pub fn list(
    conn: &Connection,
    seller_id: Option<i64>,
    status: &str,
    category_id: Option<i64>,
) -> Result<Vec<Product>> {
    let mut sql = String::from("SELECT * FROM products WHERE 1=1");
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(sid) = seller_id {
        sql.push_str(" AND seller_id = ?");
        param_values.push(Box::new(sid));
    }
    if !status.is_empty() && status != "all" {
        sql.push_str(" AND status = ?");
        param_values.push(Box::new(status.to_string()));
    }
    if let Some(cid) = category_id {
        sql.push_str(" AND category_id = ?");
        param_values.push(Box::new(cid));
    }
    sql.push_str(" ORDER BY id");
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Product {
            id: row.get(0)?,
            seller_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            category_id: row.get(6)?,
            status: row.get(7)?,
            sales_count: row.get(8)?,
            rating: row.get(9)?,
            review_count: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;
    let mut products = Vec::new();
    for row in rows {
        products.push(row?);
    }
    Ok(products)
}

pub fn search(
    conn: &Connection,
    keyword: &str,
    category_id: Option<i64>,
    min_price: Option<i64>,
    max_price: Option<i64>,
    seller_id: Option<i64>,
) -> Result<Vec<Product>> {
    let mut sql = String::from("SELECT * FROM products WHERE status = 'active'");
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if !keyword.is_empty() {
        sql.push_str(" AND (title LIKE ? OR description LIKE ?)");
        let kw = format!("%{}%", keyword);
        param_values.push(Box::new(kw.clone()));
        param_values.push(Box::new(kw));
    }
    if let Some(cid) = category_id {
        sql.push_str(" AND category_id = ?");
        param_values.push(Box::new(cid));
    }
    if let Some(p) = min_price {
        sql.push_str(" AND price >= ?");
        param_values.push(Box::new(p));
    }
    if let Some(p) = max_price {
        sql.push_str(" AND price <= ?");
        param_values.push(Box::new(p));
    }
    if let Some(sid) = seller_id {
        sql.push_str(" AND seller_id = ?");
        param_values.push(Box::new(sid));
    }
    sql.push_str(" ORDER BY id");
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Product {
            id: row.get(0)?,
            seller_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            category_id: row.get(6)?,
            status: row.get(7)?,
            sales_count: row.get(8)?,
            rating: row.get(9)?,
            review_count: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;
    let mut products = Vec::new();
    for row in rows {
        products.push(row?);
    }
    Ok(products)
}

pub fn get(conn: &Connection, id: i64) -> Result<Product> {
    let mut stmt = conn.prepare("SELECT * FROM products WHERE id = ?1")?;
    let product = stmt.query_row(params![id], |row| {
        Ok(Product {
            id: row.get(0)?,
            seller_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            category_id: row.get(6)?,
            status: row.get(7)?,
            sales_count: row.get(8)?,
            rating: row.get(9)?,
            review_count: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;
    Ok(product)
}

pub fn update(
    conn: &Connection,
    id: i64,
    title: &str,
    price: i64,
    stock: i64,
    status: &str,
    description: &str,
    category_id: Option<i64>,
) -> Result<Product> {
    if !["active", "inactive", "deleted"].contains(&status) {
        bail!("狀態必須為 active、inactive 或 deleted");
    }
    if title.trim().is_empty() {
        bail!("商品名稱不能為空");
    }
    if price < 0 {
        bail!("價格不能為負數");
    }
    if stock < 0 {
        bail!("庫存不能為負數");
    }
    let affected = conn.execute(
        "UPDATE products SET title = ?1, price = ?2, stock = ?3, status = ?4, description = ?5, category_id = ?6, updated_at = datetime('now') WHERE id = ?7",
        params![title.trim(), price, stock, status, description, category_id, id],
    )?;
    if affected == 0 {
        bail!("商品不存在");
    }
    get(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM products WHERE id = ?1", params![id])?;
    if affected == 0 {
        bail!("商品不存在");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::model::user;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        user::add(&conn, "seller", "賣家", "seller", "", "", "").unwrap();
        conn
    }

    #[test]
    fn test_add_and_get_product() {
        let conn = setup();
        let p = add(&conn, 1, "測試商品", 100, 10, "一個測試商品", None).unwrap();
        assert_eq!(p.title, "測試商品");
        assert_eq!(p.price, 100);
        assert_eq!(p.stock, 10);
        assert_eq!(p.status, "active");
        assert!(p.category_id.is_none());

        let got = get(&conn, p.id).unwrap();
        assert_eq!(got.title, "測試商品");
    }

    #[test]
    fn test_add_product_with_category() {
        let conn = setup();
        crate::model::category::add(&conn, "3C", None).unwrap();
        let p = add(&conn, 1, "手機", 100, 10, "", Some(1)).unwrap();
        assert_eq!(p.category_id, Some(1));
    }

    #[test]
    fn test_add_product_invalid_price() {
        let conn = setup();
        let r = add(&conn, 1, "壞商品", -1, 10, "", None);
        assert!(r.is_err());
    }

    #[test]
    fn test_add_product_empty_title() {
        let conn = setup();
        let r = add(&conn, 1, "  ", 100, 10, "", None);
        assert!(r.is_err());
    }

    #[test]
    fn test_list_products() {
        let conn = setup();
        add(&conn, 1, "商品A", 100, 10, "", None).unwrap();
        add(&conn, 1, "商品B", 200, 5, "", None).unwrap();
        let products = list(&conn, None, "all", None).unwrap();
        assert_eq!(products.len(), 2);
    }

    #[test]
    fn test_update_product() {
        let conn = setup();
        let p = add(&conn, 1, "舊商品", 100, 10, "", None).unwrap();
        update(&conn, p.id, "新商品", 200, 20, "inactive", "新描述", None).unwrap();
        let got = get(&conn, p.id).unwrap();
        assert_eq!(got.title, "新商品");
        assert_eq!(got.price, 200);
        assert_eq!(got.status, "inactive");
    }

    #[test]
    fn test_delete_product() {
        let conn = setup();
        let p = add(&conn, 1, "商品", 100, 10, "", None).unwrap();
        delete(&conn, p.id).unwrap();
        let r = get(&conn, p.id);
        assert!(r.is_err());
    }

    #[test]
    fn test_search_products() {
        let conn = setup();
        add(&conn, 1, "無線耳機", 500, 10, "藍牙耳機", None).unwrap();
        add(&conn, 1, "充電線", 200, 10, "USB-C", None).unwrap();
        let r = search(&conn, "耳機", None, None, None, None).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].title, "無線耳機");
    }

    #[test]
    fn test_search_by_price_range() {
        let conn = setup();
        add(&conn, 1, "便宜商品", 100, 10, "", None).unwrap();
        add(&conn, 1, "中等商品", 500, 10, "", None).unwrap();
        add(&conn, 1, "昂貴商品", 1000, 10, "", None).unwrap();
        let r = search(&conn, "", None, Some(200), Some(800), None).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].title, "中等商品");
    }
}
