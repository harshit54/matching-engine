use rust_decimal::Decimal;

use crate::{
    domain::order::model::{Side, Type},
    repo::error::RepoError,
};

pub struct OrderDbModel {
    pub id: i32,
    pub user_id: i32,
    pub instrument_id: i32,
    pub order_type: Type,
    pub order_side: Side,
    pub quantity: Decimal,
    pub price: Option<Decimal>, // Only required for Limit orders
}

pub trait OrderRepo {
    fn create_order(&self, username: String) -> Result<OrderDbModel, RepoError>;
    fn get_order(&self, id: i32) -> Result<OrderDbModel, RepoError>;
    fn list_orders(&self) -> Result<Vec<OrderDbModel>, RepoError>;
}
