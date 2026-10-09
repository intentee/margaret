use std::num::NonZeroUsize;
use std::str::FromStr;

use crate::database_error::DatabaseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaxConnections {
    pub connections: NonZeroUsize,
}

impl FromStr for MaxConnections {
    type Err = DatabaseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        NonZeroUsize::from_str(value)
            .map(|connections| Self { connections })
            .map_err(DatabaseError::MalformedMaxConnections)
    }
}

#[cfg(test)]
mod tests {
    use super::MaxConnections;

    #[test]
    fn reads_a_positive_number_of_connections() {
        assert_eq!(
            "3".parse::<MaxConnections>()
                .expect("three connections are a pool size")
                .connections
                .get(),
            3
        );
    }

    #[test]
    fn rejects_a_pool_without_connections() {
        assert_eq!(
            "0".parse::<MaxConnections>()
                .expect_err("a pool needs a connection")
                .to_string(),
            "the maximum number of database connections is malformed: number would be zero for non-zero type"
        );
    }
}
