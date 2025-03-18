#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use crate::{
        common::error::DbError, contracts::connection::DbConnection,
        modules::pgsql::connection::PostgresConnection,
    };

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
