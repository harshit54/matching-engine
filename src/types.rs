#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug)]
pub struct Trade {
    pub id: u64,
    pub maker_oid: u64,
    pub taker_oid: u64,
    pub price: u64,
    pub quantity: u64,
}
