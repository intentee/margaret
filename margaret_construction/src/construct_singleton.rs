use std::sync::Arc;

use crate::construction_error::ConstructionError;

/// # Errors
///
/// Returns [`ConstructionError`] naming `singleton` when its constructor fails.
pub fn construct_singleton<Constructed>(
    singleton: &'static str,
    outcome: anyhow::Result<Constructed>,
) -> Result<Arc<Constructed>, ConstructionError> {
    ConstructionError::wrap(singleton, outcome).map(Arc::new)
}

#[cfg(test)]
mod tests {
    use super::construct_singleton;

    #[test]
    fn shares_a_constructed_singleton_behind_a_reference_count() {
        let constructed = construct_singleton("crate::worker::Worker", Ok(7))
            .expect("a successful outcome is shared");

        assert_eq!(*constructed, 7);
    }

    #[test]
    fn names_the_singleton_that_failed_to_construct() {
        let outcome: anyhow::Result<u8> = Err(anyhow::anyhow!("socket refused"));
        let error = construct_singleton("crate::worker::Worker", outcome)
            .expect_err("a failing outcome is reported");

        assert!(error.to_string().contains("crate::worker::Worker"));
    }
}
