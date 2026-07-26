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
    pub fn wrap<Constructed, Cause>(
        singleton: &'static str,
        outcome: Result<Constructed, Cause>,
    ) -> Result<Constructed, Self>
    where
        Cause: Into<anyhow::Error>,
    {
        outcome.map_err(|cause| Self::UserError {
            singleton,
            source: cause.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    use super::ConstructionError;

    #[test]
    fn passes_a_successful_outcome_through_unchanged() {
        let outcome: Result<u8, anyhow::Error> = Ok(7);

        assert_eq!(
            ConstructionError::wrap("crate::worker::Worker", outcome).ok(),
            Some(7)
        );
    }

    #[test]
    fn wraps_an_anyhow_failure_and_exposes_the_full_cause_chain() {
        let outcome: Result<u8, anyhow::Error> =
            Err(anyhow::anyhow!("socket refused").context("opening the ledger"));
        let error = ConstructionError::wrap("crate::worker::Worker", outcome)
            .expect_err("a failing outcome is wrapped");

        let rendered = error.to_string();

        assert!(rendered.contains("crate::worker::Worker"));
        assert!(rendered.contains("opening the ledger"));
        assert!(rendered.contains("socket refused"));
        assert!(error.source().is_some());
    }

    #[test]
    fn wraps_a_standard_library_failure() {
        let outcome: Result<u8, IoError> =
            Err(IoError::new(ErrorKind::NotFound, "the key file is missing"));
        let error = ConstructionError::wrap("crate::keys::KeyLoader", outcome)
            .expect_err("a failing outcome is wrapped");

        assert!(error.to_string().contains("the key file is missing"));
    }
}
