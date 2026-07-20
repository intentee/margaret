use thiserror::Error;

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("the endpoint could not be resolved: {source}")]
    Unresolved {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl EndpointError {
    #[must_use]
    pub fn unresolved(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Unresolved {
            source: Box::new(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fmt::Result as FormatResult;

    use super::EndpointError;

    #[derive(Debug)]
    struct Cause;

    impl Display for Cause {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
            write!(formatter, "the underlying cause")
        }
    }

    impl std::error::Error for Cause {}

    #[test]
    fn unresolved_wraps_and_displays_the_source() {
        let error = EndpointError::unresolved(Cause);

        assert_eq!(
            error.to_string(),
            "the endpoint could not be resolved: the underlying cause"
        );
    }
}
