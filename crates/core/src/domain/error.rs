pub enum ServiceError {
    NotFoundError,
    DatabaseError,
    InvalidUserData(String),
}