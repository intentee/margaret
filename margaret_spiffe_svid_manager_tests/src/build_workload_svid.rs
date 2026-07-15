use spiffe::X509Svid;

use crate::ca_der::CA_DER;
use crate::leaf_spiffe_example_org_workload_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;
use crate::leaf_spiffe_example_org_workload_key_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

#[must_use]
pub fn build_workload_svid() -> X509Svid {
    let cert_chain_der = [LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER, CA_DER].concat();

    X509Svid::parse_from_der(&cert_chain_der, LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER).unwrap()
}
