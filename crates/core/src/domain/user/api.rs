use log::error;

use crate::domain::error::ServiceError::DatabaseError;
use crate::{
    domain::{error::ServiceError, user::User},
    repo::user::UserRepo,
};

pub trait Service {
    fn create_user(&self, username: String) -> Result<User, ServiceError>;
    fn get_user(&self, id: i32) -> Result<User, ServiceError>;
    fn list_users(&self) -> Result<Vec<User>, ServiceError>;
}

pub struct ServiceImpl<R: UserRepo> {
    repo: R,
}

impl<R: UserRepo> ServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: UserRepo> Service for ServiceImpl<R> {
    fn create_user(&self, username: String) -> Result<User, ServiceError> {
        match self.repo.create_user(username) {
            Ok(val) => Ok(User {
                id: val.id,
                username: val.username,
            }),
            Err(err) => {
                error!("couldn't fetch result from user repo - {:?}", err);
                Err(DatabaseError)
            }
        }
    }

    fn get_user(&self, id: i32) -> Result<User, ServiceError> {
        match self.repo.get_user(id) {
            Ok(val) => Ok(User {
                id: val.id,
                username: val.username,
            }),
            Err(err) => match err {
                crate::repo::error::RepoError::NotFoundError => Err(ServiceError::NotFoundError),
                crate::repo::error::RepoError::DatabaseError => Err(ServiceError::DatabaseError),
            },
        }
    }

    fn list_users(&self) -> Result<Vec<User>, ServiceError> {
        match self.repo.list_users() {
            Ok(val) => {
                let mut res: Vec<User> = Vec::with_capacity(val.len());

                for user in val.into_iter() {
                    res.push(User {
                        id: user.id,
                        username: user.username,
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
