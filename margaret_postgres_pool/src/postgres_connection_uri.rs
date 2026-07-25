use std::str::FromStr;

use sqlx::postgres::PgConnectOptions;

use crate::postgres_pool_error::PostgresPoolError;

#[derive(Clone, Debug)]
pub struct PostgresConnectionUri {
    connect_options: PgConnectOptions,
}

impl PostgresConnectionUri {
    #[must_use]
    pub fn into_connect_options(self) -> PgConnectOptions {
        self.connect_options
    }
}

impl FromStr for PostgresConnectionUri {
    type Err = PostgresPoolError;

    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            connect_options: uri
                .parse::<PgConnectOptions>()
                .map_err(|source| PostgresPoolError::InvalidConnectionUri { source })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::PostgresConnectionUri;

    #[test]
    fn parses_a_valid_postgres_url_into_connect_options() {
        let uri: PostgresConnectionUri = "postgres://user:secret@localhost:5432/app"
            .parse()
            .expect("a valid postgres URL parses");

        assert_eq!(uri.into_connect_options().get_database(), Some("app"));
    }

    #[test]
    fn rejects_a_malformed_url_with_the_invalid_connection_uri_variant() {
        let error = "not-a-postgres-url"
            .parse::<PostgresConnectionUri>()
            .expect_err("a malformed URL is rejected");

        assert!(error.to_string().contains("connection URL is invalid"));
        assert!(error.source().is_some());
    }
}
