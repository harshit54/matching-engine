use log::error;

use crate::domain::error::ServiceError::DatabaseError;
use crate::{
    domain::{error::ServiceError, instrument::Instrument},
    repo::instrument::InstrumentRepo,
};

pub trait Service {
    fn create_instrument(
        &self,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<Instrument, ServiceError>;
    fn get_instrument(&self, id: i32) -> Result<Instrument, ServiceError>;
    fn list_instruments(&self) -> Result<Vec<Instrument>, ServiceError>;
}

pub struct ServiceImpl<R: InstrumentRepo> {
    repo: R,
}

impl<R: InstrumentRepo> ServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: InstrumentRepo> Service for ServiceImpl<R> {
    fn create_instrument(
        &self,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<Instrument, ServiceError> {
        match self.repo.create_instrument(base_asset, quote_asset) {
            Ok(val) => Ok(Instrument {
                id: val.id,
                base_asset: val.base_asset,
                quote_asset: val.quote_asset,
            }),
            Err(err) => {
                error!("couldn't fetch result from user repo - {:?}", err);
                Err(DatabaseError)
            }
        }
    }

    fn get_instrument(&self, id: i32) -> Result<Instrument, ServiceError> {
        match self.repo.get_instrument(id) {
            Ok(val) => Ok(Instrument {
                id: val.id,
                base_asset: val.base_asset,
                quote_asset: val.quote_asset,
            }),
            Err(err) => match err {
                crate::repo::error::RepoError::NotFoundError => Err(ServiceError::NotFoundError),
                crate::repo::error::RepoError::DatabaseError => Err(ServiceError::DatabaseError),
            },
        }
    }

    fn list_instruments(&self) -> Result<Vec<Instrument>, ServiceError> {
        match self.repo.list_instruments() {
            Ok(val) => {
                let mut res: Vec<Instrument> = Vec::with_capacity(val.len());

                for instrument in val.into_iter() {
                    res.push(Instrument {
                        id: instrument.id,
                        base_asset: instrument.base_asset,
                        quote_asset: instrument.quote_asset,
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
