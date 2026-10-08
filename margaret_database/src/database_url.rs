use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::FromStr;

use tokio_postgres::Config;

use crate::database_error::DatabaseError;

#[derive(Clone)]
pub struct DatabaseUrl {
    pub(crate) config: Config,
}

impl Debug for DatabaseUrl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("DatabaseUrl")
            .finish_non_exhaustive()
    }
}

impl FromStr for DatabaseUrl {
    type Err = DatabaseError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Config::from_str(value)
            .map(|config| Self { config })
            .map_err(DatabaseError::MalformedUrl)
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseUrl;
    use crate::database_error::DatabaseError;

    #[test]
    fn reads_the_connection_of_a_postgres_url() {
        let url: DatabaseUrl = "postgresql://margaret:secret@localhost:5432/blog"
            .parse()
            .expect("the url is a postgres url");

        assert_eq!(url.config.get_dbname(), Some("blog"));
        assert_eq!(url.config.get_user(), Some("margaret"));
    }

    #[test]
    fn rejects_a_value_that_is_not_a_postgres_url() {
        assert!(matches!(
            "postgresql://localhost:not-a-port/blog".parse::<DatabaseUrl>(),
            Err(DatabaseError::MalformedUrl(_))
        ));
    }

    #[test]
    fn keeps_the_password_out_of_its_debug_output() {
        let url: DatabaseUrl = "postgresql://margaret:secret@localhost/blog"
            .parse()
            .expect("the url is a postgres url");

        assert_eq!(format!("{url:?}"), "DatabaseUrl { .. }");
    }
}
