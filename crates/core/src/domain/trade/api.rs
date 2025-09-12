use log::error;
use rust_decimal::Decimal;

use crate::domain::error::ServiceError::DatabaseError;
use crate::{
    domain::{error::ServiceError, trade::Trade},
    repo::trade::TradeRepo,
};

pub trait Service {
    fn create_trade(
        &self,
        buy_order_id: i32,
        sell_order_id: i32,
        price: &Decimal,
        qty: &Decimal,
    ) -> Result<Trade, ServiceError>;
    fn get_trade(&self, id: i32) -> Result<Trade, ServiceError>;
    fn list_trades(&self) -> Result<Vec<Trade>, ServiceError>;
}

pub struct ServiceImpl<R: TradeRepo> {
    repo: R,
}

impl<R: TradeRepo> ServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TradeRepo> Service for ServiceImpl<R> {
    fn create_trade(
        &self,
        buy_order_id: i32,
        sell_order_id: i32,
        price: &Decimal,
        qty: &Decimal,
    ) -> Result<Trade, ServiceError> {
        match self
            .repo
            .create_trade(buy_order_id, sell_order_id, price, qty)
        {
            Ok(val) => Ok(Trade {
                id: val.id,
                price: val.price,
                qty: val.qty,
                buy_order_id: val.buy_order_id,
                sell_order_id: val.sell_order_id,
                created_at: val.created_at,
            }),
            Err(err) => {
                error!("couldn't fetch result from user repo - {:?}", err);
                Err(DatabaseError)
            }
        }
    }

    fn get_trade(&self, id: i32) -> Result<Trade, ServiceError> {
        match self.repo.get_trade(id) {
            Ok(val) => Ok(Trade {
                id: val.id,
                price: val.price,
                qty: val.qty,
                buy_order_id: val.buy_order_id,
                sell_order_id: val.sell_order_id,
                created_at: val.created_at,
            }),
            Err(err) => match err {
                crate::repo::error::RepoError::NotFoundError => Err(ServiceError::NotFoundError),
                crate::repo::error::RepoError::DatabaseError => Err(ServiceError::DatabaseError),
            },
        }
    }

    fn list_trades(&self) -> Result<Vec<Trade>, ServiceError> {
        match self.repo.list_trades() {
            Ok(val) => {
                let mut res: Vec<Trade> = Vec::with_capacity(val.len());

                for trade in val.into_iter() {
                    res.push(Trade {
                        id: trade.id,
                        price: trade.price,
                        qty: trade.qty,
                        buy_order_id: trade.buy_order_id,
                        sell_order_id: trade.sell_order_id,
                        created_at: trade.created_at,
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
