use rustls::pki_types::CertificateDer;

use crate::ca_der::CA_DER;
use crate::leaf_spiffe_example_org_workload_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;

#[must_use]
pub fn build_workload_cert_chain() -> Vec<CertificateDer<'static>> {
    vec![
        CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER.to_vec()),
        CertificateDer::from(CA_DER.to_vec()),
    ]
}
