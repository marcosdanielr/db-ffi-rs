use postgres::{Client, NoTls};

use crate::{common::error::DbError, contracts::connection::DbConnection};

pub struct PostgresConnection {
    client: Client,
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

#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use super::*;
    use crate::common::error::DbError;

    #[test]
    fn test_connect_success() {
        let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
        let connection_url_str: &str = connection_url.as_str();

        let result = PostgresConnection::connect(connection_url_str);

        match result {
            Ok(mut conn) => {
                let timeout = Duration::from_secs(5);
                assert!(conn.client.is_valid(timeout).is_ok());
            }
            Err(err) => {
                panic!("Connection failed: {}", err);
            }
        }
    }

    #[test]
    fn test_connect_failure() {
        let invalid_connection_string =
            "postgres://wronguser:wrongpassword@invalidhost:5432/nonexistentdb";

        let result = PostgresConnection::connect(invalid_connection_string);

        match result {
            Ok(_) => panic!("Connection should have failed"),
            Err(err) => {
                assert!(matches!(err, DbError::ConnectionFailed(_)));
            }
        }
    }
}
