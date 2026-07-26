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
    pub fn wrap<Constructed>(
        singleton: &'static str,
        outcome: anyhow::Result<Constructed>,
    ) -> Result<Constructed, Self> {
        outcome.map_err(|source| Self::UserError { singleton, source })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::ConstructionError;

    #[test]
    fn passes_a_successful_outcome_through_unchanged() {
        let outcome: anyhow::Result<u8> = Ok(7);

        assert_eq!(
            ConstructionError::wrap("crate::worker::Worker", outcome).ok(),
            Some(7)
        );
    }

    #[test]
    fn wraps_a_failure_and_exposes_the_full_cause_chain() {
        let outcome: anyhow::Result<u8> =
            Err(anyhow::anyhow!("socket refused").context("opening the ledger"));
        let error = ConstructionError::wrap("crate::worker::Worker", outcome)
            .expect_err("a failing outcome is wrapped");

        let rendered = error.to_string();

        assert!(rendered.contains("crate::worker::Worker"));
        assert!(rendered.contains("opening the ledger"));
        assert!(rendered.contains("socket refused"));
        assert!(error.source().is_some());
    }
}
