use crate::repo::error::RepoError;

pub struct UserDbModel {
    pub id: i32,
    pub username: String,
}

pub trait UserRepo {
    fn create_user(&self, username: String) -> Result<UserDbModel, RepoError>;
    fn get_user(&self, id: i32) -> Result<UserDbModel, RepoError>;
    fn list_users(&self) -> Result<Vec<UserDbModel>, RepoError>;
}
