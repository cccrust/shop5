use anyhow::{Result, bail};
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Review {
    pub id: i64,
    pub order_id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub rating: i64,
    pub content: String,
    pub created_at: String,
}

fn update_product_rating(conn: &Connection, product_id: i64) -> Result<()> {
    let (avg, cnt): (f64, i64) = conn.query_row(
        "SELECT COALESCE(AVG(CAST(rating AS REAL)), 0), COUNT(*) FROM reviews WHERE product_id = ?1",
        params![product_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    conn.execute(
        "UPDATE products SET rating = ?1, review_count = ?2 WHERE id = ?3",
        params![avg, cnt, product_id],
    )?;
    Ok(())
}

pub fn add(
    conn: &Connection,
    order_id: i64,
    user_id: i64,
    product_id: i64,
    rating: i64,
    content: &str,
) -> Result<Review> {
    if rating < 1 || rating > 5 {
        bail!("評分必須在 1 到 5 之間");
    }

    let order_status: String = conn.query_row(
        "SELECT status FROM orders WHERE id = ?1",
        params![order_id],
        |row| row.get(0),
    )?;
    if order_status != "delivered" {
        bail!("只能對已送達的訂單進行評價");
    }

    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM order_items WHERE order_id = ?1 AND product_id = ?2",
        params![order_id, product_id],
        |row| row.get(0),
    )?;
    if !exists {
        bail!("此訂單中沒有該商品");
    }

    let order_buyer_id: i64 = conn.query_row(
        "SELECT buyer_id FROM orders WHERE id = ?1",
        params![order_id],
        |row| row.get(0),
    )?;
    if order_buyer_id != user_id {
        bail!("只有訂單買家可以評價");
    }

    conn.execute(
        "INSERT INTO reviews (order_id, user_id, product_id, rating, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![order_id, user_id, product_id, rating, content.trim()],
    )?;
    let id = conn.last_insert_rowid();
    update_product_rating(conn, product_id)?;
    get(conn, id)
}

pub fn get(conn: &Connection, id: i64) -> Result<Review> {
    let mut stmt = conn.prepare("SELECT * FROM reviews WHERE id = ?1")?;
    let review = stmt.query_row(params![id], |row| {
        Ok(Review {
            id: row.get(0)?,
            order_id: row.get(1)?,
            user_id: row.get(2)?,
            product_id: row.get(3)?,
            rating: row.get(4)?,
            content: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    Ok(review)
}

pub fn list_by_product(conn: &Connection, product_id: i64) -> Result<Vec<Review>> {
    let mut stmt = conn.prepare("SELECT * FROM reviews WHERE product_id = ?1 ORDER BY created_at DESC")?;
    let rows = stmt.query_map(params![product_id], |row| {
        Ok(Review {
            id: row.get(0)?,
            order_id: row.get(1)?,
            user_id: row.get(2)?,
            product_id: row.get(3)?,
            rating: row.get(4)?,
            content: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut reviews = Vec::new();
    for row in rows {
        reviews.push(row?);
    }
    Ok(reviews)
}

pub fn list_by_user(conn: &Connection, user_id: i64) -> Result<Vec<Review>> {
    let mut stmt = conn.prepare("SELECT * FROM reviews WHERE user_id = ?1 ORDER BY created_at DESC")?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Review {
            id: row.get(0)?,
            order_id: row.get(1)?,
            user_id: row.get(2)?,
            product_id: row.get(3)?,
            rating: row.get(4)?,
            content: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut reviews = Vec::new();
    for row in rows {
        reviews.push(row?);
    }
    Ok(reviews)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let product_id: i64 = conn.query_row(
        "SELECT product_id FROM reviews WHERE id = ?1",
        params![id],
        |row| row.get(0),
    ).map_err(|_| anyhow::anyhow!("評價不存在"))?;
    let affected = conn.execute("DELETE FROM reviews WHERE id = ?1", params![id])?;
    if affected == 0 {
        bail!("評價不存在");
    }
    update_product_rating(conn, product_id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::model::user;
    use crate::model::product;
    use crate::model::order;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        user::add(&conn, "seller1", "賣家A", "seller", "", "", "").unwrap();
        user::add(&conn, "buyer1", "買家B", "buyer", "", "", "").unwrap();
        product::add(&conn, 1, "商品1", 100, 10, "測試", None).unwrap();
        product::add(&conn, 1, "商品2", 200, 10, "測試", None).unwrap();
        order::create_direct(&conn, 2, 1, &[(1, 1), (2, 1)], "").unwrap();
        conn.execute("UPDATE orders SET status = 'delivered' WHERE id = 1", []).unwrap();
        conn
    }

    #[test]
    fn test_add_review() {
        let conn = setup();
        let r = add(&conn, 1, 2, 1, 5, "很棒！").unwrap();
        assert_eq!(r.rating, 5);
        assert_eq!(r.content, "很棒！");

        let p = product::get(&conn, 1).unwrap();
        assert_eq!(p.rating, 5.0);
        assert_eq!(p.review_count, 1);
    }

    #[test]
    fn test_add_review_invalid_rating() {
        let conn = setup();
        let r = add(&conn, 1, 2, 1, 6, "");
        assert!(r.is_err());
        let r = add(&conn, 1, 2, 1, 0, "");
        assert!(r.is_err());
    }

    #[test]
    fn test_add_review_not_delivered() {
        let conn = setup();
        order::create_direct(&conn, 2, 1, &[(1, 1)], "").unwrap();
        let r = add(&conn, 2, 2, 1, 4, "");
        assert!(r.is_err());
    }

    #[test]
    fn test_list_by_product() {
        let conn = setup();
        add(&conn, 1, 2, 1, 5, "讚").unwrap();
        add(&conn, 1, 2, 2, 4, "好").unwrap();
        let reviews = list_by_product(&conn, 1).unwrap();
        assert_eq!(reviews.len(), 1);
    }

    #[test]
    fn test_list_by_user() {
        let conn = setup();
        add(&conn, 1, 2, 1, 5, "").unwrap();
        add(&conn, 1, 2, 2, 4, "").unwrap();
        let reviews = list_by_user(&conn, 2).unwrap();
        assert_eq!(reviews.len(), 2);
    }

    #[test]
    fn test_delete_review_updates_rating() {
        let conn = setup();
        let r = add(&conn, 1, 2, 1, 5, "").unwrap();
        delete(&conn, r.id).unwrap();
        let p = product::get(&conn, 1).unwrap();
        assert_eq!(p.rating, 0.0);
        assert_eq!(p.review_count, 0);
    }

    #[test]
    fn test_rating_average() {
        let conn = setup();
        add(&conn, 1, 2, 1, 5, "").unwrap();
        order::create_direct(&conn, 2, 1, &[(1, 1)], "").unwrap();
        conn.execute("UPDATE orders SET status = 'delivered' WHERE id = 2", []).unwrap();
        add(&conn, 2, 2, 1, 1, "").unwrap();
        let p = product::get(&conn, 1).unwrap();
        assert!((p.rating - 3.0).abs() < 0.01);
        assert_eq!(p.review_count, 2);
    }
}
