use crate::model::review::Review;
use rusqlite::Connection;

pub fn fmt_user(conn: &Connection, u: &crate::model::user::User) {
    println!("#{} {} (@{})", u.id, u.display_name, u.username);
    println!("   角色: {}", u.role);
    if !u.bio.is_empty() {
        println!("   簡介: {}", u.bio);
    }
    println!("   建立: {}", u.created_at);
}

pub fn fmt_user_brief(u: &crate::model::user::User) {
    println!("  #{} {} (@{}) · {}", u.id, u.display_name, u.username, u.role);
}

pub fn fmt_product(p: &crate::model::product::Product) {
    let stars = if p.review_count > 0 {
        format!(" ★{:.1}/5 ({}人)", p.rating, p.review_count)
    } else {
        String::new()
    };
    println!(
        "#{} {} — NT${} / 庫存 {} / {}{}",
        p.id, p.title, p.price, p.stock, p.status, stars
    );
    if !p.description.is_empty() {
        println!("   描述: {}", p.description);
    }
    let cat = match p.category_id {
        Some(cid) => format!("分類: #{}", cid),
        None => "未分類".to_string(),
    };
    println!("   賣家: #{} | 銷售: {} | {} | 建立: {}", p.seller_id, p.sales_count, cat, p.created_at);
}

pub fn fmt_category(c: &crate::model::category::Category) {
    match c.parent_id {
        Some(pid) => println!("#{} {} (上層: #{})", c.id, c.name, pid),
        None => println!("#{} {} (頂層)", c.id, c.name),
    }
}

pub fn fmt_cart_item(item: &crate::model::cart::CartItemWithProduct) {
    println!(
        "  #{} {} x{} — NT${} (庫存 {})",
        item.product_id, item.title, item.quantity, item.price, item.stock
    );
}

pub fn fmt_order(conn: &Connection, o: &crate::model::order::Order) {
    println!(
        "訂單 #{} — NT${} — {}",
        o.id, o.total, o.status
    );
    println!("   買家: #{} | 賣家: #{} | 建立: {}", o.buyer_id, o.seller_id, o.created_at);
    if !o.note.is_empty() {
        println!("   備註: {}", o.note);
    }
}

pub fn fmt_order_item(oi: &crate::model::order::OrderItem) {
    println!(
        "    {} x{} — NT${}",
        oi.product_title, oi.quantity, oi.subtotal
    );
}

pub fn fmt_review(conn: &Connection, r: &Review) {
    let username: String = conn
        .query_row(
            "SELECT display_name FROM users WHERE id = ?1",
            rusqlite::params![r.user_id],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| format!("#{}", r.user_id));
    let stars = "★".repeat(r.rating as usize) + &"☆".repeat((5 - r.rating) as usize);
    println!(
        "#{} {} {} {}",
        r.id, username, stars, r.created_at
    );
    if !r.content.is_empty() {
        println!("   \"{}\"", r.content);
    }
}
