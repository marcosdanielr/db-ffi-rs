use crate::common::error::DbError;

pub trait DbConnection {
    fn connect(connection_string: &str) -> Result<Self, DbError>
    where
        Self: Sized;
    fn close(&self) -> Result<(), DbError>;
}
