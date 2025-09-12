use std::time;

use rust_decimal::Decimal;

pub struct Trade {
    pub id: i32,
    pub buy_order_id: i32,
    pub sell_order_id: i32,
    pub price: Decimal,
    pub qty: Decimal,
    pub created_at: time::Instant,
}
