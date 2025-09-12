use log::error;
use rust_decimal::Decimal;

use crate::domain::error::ServiceError::DatabaseError;
use crate::domain::order::model::{Side, Type};
use crate::{
    domain::{
        error::ServiceError,
        order::{Order, model::State},
    },
    repo::order::OrderRepo,
};

pub trait Service {
    fn create_order(
        &self,
        user_id: i32,
        instrument_id: i32,
        order_state: State,
        order_type: Type,
        order_side: Side,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Result<Order, ServiceError>;
    fn get_order(&self, id: i32) -> Result<Order, ServiceError>;
    fn list_orders(&self) -> Result<Vec<Order>, ServiceError>;
}

pub struct ServiceImpl<R: OrderRepo> {
    repo: R,
}

impl<R: OrderRepo> ServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: OrderRepo> Service for ServiceImpl<R> {
    fn create_order(
        &self,
        user_id: i32,
        instrument_id: i32,
        order_state: State,
        order_type: Type,
        order_side: Side,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Result<Order, ServiceError> {
        match self.repo.create_order(
            user_id,
            instrument_id,
            order_state,
            order_type,
            order_side,
            quantity,
            price,
        ) {
            Ok(val) => Ok(Order {
                id: val.id,
                user_id: val.user_id,
                instrument_id: val.instrument_id,
                order_state: State::Created,
                order_side: val.order_side,
                order_type: val.order_type,
                price: val.price,
                quantity: val.quantity,
            }),
            Err(err) => {
                error!("couldn't fetch result from user repo - {:?}", err);
                Err(DatabaseError)
            }
        }
    }

    fn get_order(&self, id: i32) -> Result<Order, ServiceError> {
        match self.repo.get_order(id) {
            Ok(val) => Ok(Order {
                id: val.id,
                user_id: val.user_id,
                instrument_id: val.instrument_id,
                order_state: val.order_state,
                order_side: val.order_side,
                order_type: val.order_type,
                price: val.price,
                quantity: val.quantity,
            }),
            Err(err) => match err {
                crate::repo::error::RepoError::NotFoundError => Err(ServiceError::NotFoundError),
                crate::repo::error::RepoError::DatabaseError => Err(ServiceError::DatabaseError),
            },
        }
    }

    fn list_orders(&self) -> Result<Vec<Order>, ServiceError> {
        match self.repo.list_orders() {
            Ok(val) => {
                let mut res: Vec<Order> = Vec::with_capacity(val.len());

                for order in val.into_iter() {
                    res.push(Order {
                        id: order.id,
                        user_id: order.user_id,
                        instrument_id: order.instrument_id,
                        order_side: order.order_side,
                        order_type: order.order_type,
                        order_state: order.order_state,
                        price: order.price,
                        quantity: order.quantity,
                    });
                }

                Ok(res)
            }
            Err(err) => match err {
                crate::repo::error::RepoError::NotFoundError => Err(ServiceError::NotFoundError),
                crate::repo::error::RepoError::DatabaseError => Err(ServiceError::DatabaseError),
            },
        }
    }
}
