use postgres::{Client, NoTls};

use crate::{common::error::DbError, contracts::connection::DbConnection};

pub struct PostgresConnection {
    pub client: Client,
}

impl DbConnection for PostgresConnection {
    fn connect(connection_string: &str) -> Result<Self, DbError> {
        match Client::connect(connection_string, NoTls) {
            Ok(client) => Ok(PostgresConnection { client }),
            Err(e) => Err(DbError::ConnectionFailed(format!("{}", e))),
        }
    }

    fn close(&self) -> Result<(), DbError> {
        Ok(())
    }
}
