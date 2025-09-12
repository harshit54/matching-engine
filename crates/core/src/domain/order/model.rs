use rust_decimal::Decimal;

pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub instrument_id: i32,
    pub order_type: Type,
    pub order_side: Side,
    pub order_state: State,
    pub quantity: Decimal,
    pub price: Option<Decimal>, // Only required for Limit orders
}

pub enum Type {
    Market,
    Limit,
}

pub enum Side {
    Buy,
    Sell,
}

pub enum State {
    Created,
    PartiallyFilled,
    Filled,
}
