use spiffe::X509Svid;

use crate::constants::CA_DER;
use crate::constants::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;
use crate::constants::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_workload_svid() -> X509Svid {
    let cert_chain_der = [LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER, CA_DER].concat();

    X509Svid::parse_from_der(&cert_chain_der, LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER).unwrap()
}
