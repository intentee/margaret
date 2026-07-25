use std::num::NonZeroU32;
use std::str::FromStr;

use crate::postgres_pool_error::PostgresPoolError;

#[derive(Clone, Debug)]
pub struct PostgresMaxConnections {
    max_connections: NonZeroU32,
}

impl PostgresMaxConnections {
    #[must_use]
    pub fn get(&self) -> u32 {
        self.max_connections.get()
    }
}

impl FromStr for PostgresMaxConnections {
    type Err = PostgresPoolError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            max_connections: text
                .parse::<NonZeroU32>()
                .map_err(|source| PostgresPoolError::InvalidMaxConnections { source })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresMaxConnections;

    #[test]
    fn parses_a_positive_count() {
        let max_connections: PostgresMaxConnections =
            "5".parse().expect("5 is a valid max connections");

        assert_eq!(max_connections.get(), 5);
    }

    #[test]
    fn rejects_zero() {
        assert!("0".parse::<PostgresMaxConnections>().is_err());
    }
}
