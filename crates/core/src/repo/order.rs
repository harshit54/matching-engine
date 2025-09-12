use std::time;

use rust_decimal::Decimal;

use crate::{
    domain::order::model::{Side, State, Type},
    repo::error::RepoError,
};

pub struct OrderDbModel {
    pub id: i32,
    pub user_id: i32,
    pub instrument_id: i32,
    pub order_state: State,
    pub order_type: Type,
    pub order_side: Side,
    pub quantity: Decimal,
    pub price: Option<Decimal>, // Only required for Limit orders
    pub created_at: time::Instant,
}

pub trait OrderRepo {
    fn create_order(
        &self,
        user_id: i32,
        instrument_id: i32,
        order_state: State,
        order_type: Type,
        order_side: Side,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Result<OrderDbModel, RepoError>;

    fn update_order_state(&self, id: i32, new_state: State) -> Result<OrderDbModel, RepoError>;

    fn get_order(&self, id: i32) -> Result<OrderDbModel, RepoError>;

    fn list_orders(&self) -> Result<Vec<OrderDbModel>, RepoError>;
}
