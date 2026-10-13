use anyhow::Result;
use spiffe::X509Context;

use crate::build_x509_context::build_x509_context;
use crate::constants::CA_DER;
use crate::constants::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;
use crate::constants::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub fn build_workload_x509_context() -> Result<X509Context> {
    let cert_chain_der = [LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER, CA_DER].concat();

    build_x509_context(
        &cert_chain_der,
        LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
        "example.org",
        &[CA_DER],
    )
}

#[cfg(test)]
mod tests {
    use super::build_workload_x509_context;

    #[test]
    fn builds_the_default_workload_context_so_svid_tests_share_one_valid_fixture() {
        assert!(build_workload_x509_context().is_ok());
    }
}
