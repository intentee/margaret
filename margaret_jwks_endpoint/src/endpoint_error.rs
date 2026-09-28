use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("the endpoint could not be resolved: {source}")]
    Resolution {
        #[source]
        source: Box<dyn error::Error + Send + Sync>,
    },
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::EndpointError;

    #[test]
    fn resolution_reports_its_source() {
        let error = EndpointError::Resolution {
            source: "the srv record has no targets".into(),
        };

        assert_eq!(
            error.to_string(),
            "the endpoint could not be resolved: the srv record has no targets"
        );
        assert_eq!(
            error
                .source()
                .expect("the underlying error is preserved")
                .to_string(),
            "the srv record has no targets"
        );
    }
}
