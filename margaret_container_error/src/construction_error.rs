use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConstructionError {
    #[error("failed to construct '{singleton}': {cause:#}")]
    UserError {
        singleton: &'static str,
        cause: anyhow::Error,
    },
}

impl ConstructionError {
    #[must_use]
    pub fn user_error(singleton: &'static str, cause: impl Into<anyhow::Error>) -> Self {
        Self::UserError {
            singleton,
            cause: cause.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    use super::ConstructionError;

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
