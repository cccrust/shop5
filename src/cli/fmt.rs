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
    println!(
        "#{} {} — NT${} / 庫存 {} / {}",
        p.id, p.title, p.price, p.stock, p.status
    );
    if !p.description.is_empty() {
        println!("   描述: {}", p.description);
    }
    println!("   賣家: #{} | 銷售: {} | 建立: {}", p.seller_id, p.sales_count, p.created_at);
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
