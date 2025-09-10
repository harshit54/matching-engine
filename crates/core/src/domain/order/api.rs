use log::error;

use crate::domain::error::ServiceError::DatabaseError;
use crate::{
    domain::{error::ServiceError, order::Order},
    repo::order::OrderRepo,
};

pub trait Service {
    fn create_order(&self, username: String) -> Result<Order, ServiceError>;
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
    fn create_order(&self, username: String) -> Result<Order, ServiceError> {
        match self.repo.create_order(username) {
            Ok(val) => Ok(Order {
                id: val.id,
                user_id: val.user_id,
                instrument_id: val.instrument_id,
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

                for user in val.into_iter() {
                    res.push(Order {
                        id: user.id,
                        user_id: user.user_id,
                        instrument_id: user.instrument_id,
                        order_side: user.order_side,
                        order_type: user.order_type,
                        price: user.price,
                        quantity: user.quantity,
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
