use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConstructionError {
    #[error("failed to construct '{singleton}': {source:#}")]
    UserError {
        singleton: &'static str,
        #[source]
        source: anyhow::Error,
    },
}

impl ConstructionError {
    #[must_use]
    pub fn user_error(singleton: &'static str, source: impl Into<anyhow::Error>) -> Self {
        Self::UserError {
            singleton,
            source: source.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    use super::ConstructionError;

    #[test]
    fn exposes_the_cause_through_the_error_source_chain() {
        let cause = anyhow::anyhow!("socket refused").context("opening the ledger");
        let error = ConstructionError::user_error("crate::worker::Worker", cause);

        assert!(error.source().is_some());
    }

    #[test]
    fn renders_the_singleton_path_and_the_full_cause_chain() {
        let cause = anyhow::anyhow!("socket refused").context("opening the ledger");
        let error = ConstructionError::user_error("crate::worker::Worker", cause);

        let rendered = error.to_string();

        assert!(rendered.contains("crate::worker::Worker"));
        assert!(rendered.contains("opening the ledger"));
        assert!(rendered.contains("socket refused"));
    }

    #[test]
    fn accepts_a_standard_library_error_as_the_cause() {
        let cause = IoError::new(ErrorKind::NotFound, "the key file is missing");
        let error = ConstructionError::user_error("crate::keys::KeyLoader", cause);

        assert!(error.to_string().contains("the key file is missing"));
    }
}
