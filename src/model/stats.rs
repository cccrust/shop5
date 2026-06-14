use anyhow::Result;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyStat {
    pub date: String,
    pub order_count: i64,
    pub revenue: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TopProduct {
    pub id: i64,
    pub title: String,
    pub price: i64,
    pub sales_count: i64,
    pub total_revenue: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SellerStats {
    pub daily: Vec<DailyStat>,
    pub top_products: Vec<TopProduct>,
    pub total_orders: i64,
    pub total_revenue: i64,
    pub avg_order_value: i64,
}

pub fn seller_stats(conn: &Connection, seller_id: i64) -> Result<SellerStats> {
    let mut stmt = conn.prepare(
        "SELECT DATE(o.created_at) as day, COUNT(*) as cnt, SUM(o.total) as rev
         FROM orders o
         WHERE o.seller_id = ?1 AND o.status = 'delivered'
         GROUP BY day ORDER BY day DESC LIMIT 30",
    )?;
    let daily = stmt.query_map(params![seller_id], |row| {
        Ok(DailyStat {
            date: row.get(0)?,
            order_count: row.get(1)?,
            revenue: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.price, p.sales_count, (p.sales_count * p.price) as rev
         FROM products p
         WHERE p.seller_id = ?1
         ORDER BY p.sales_count DESC LIMIT 10",
    )?;
    let top_products = stmt.query_map(params![seller_id], |row| {
        Ok(TopProduct {
            id: row.get(0)?,
            title: row.get(1)?,
            price: row.get(2)?,
            sales_count: row.get(3)?,
            total_revenue: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    let (total_orders, total_revenue): (i64, i64) = conn.query_row(
        "SELECT COUNT(*), COALESCE(SUM(total), 0) FROM orders WHERE seller_id = ?1 AND status = 'delivered'",
        params![seller_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    let avg_order_value = if total_orders > 0 {
        total_revenue / total_orders
    } else {
        0
    };

    Ok(SellerStats {
        daily,
        top_products,
        total_orders,
        total_revenue,
        avg_order_value,
    })
}
