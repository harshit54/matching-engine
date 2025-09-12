use std::time;

use rust_decimal::Decimal;

use crate::repo::error::RepoError;

pub struct TradeDbModel {
    pub id: i32,
    pub buy_order_id: i32,
    pub sell_order_id: i32,
    pub price: Decimal,
    pub qty: Decimal,
    pub created_at: time::Instant,
}

pub trait TradeRepo {
    fn create_trade(
        &self,
        buy_order_id: i32,
        sell_order_id: i32,
        price: &Decimal,
        qty: &Decimal,
    ) -> Result<TradeDbModel, RepoError>;
    fn get_trade(&self, id: i32) -> Result<TradeDbModel, RepoError>;
    fn list_trades(&self) -> Result<Vec<TradeDbModel>, RepoError>;
}
