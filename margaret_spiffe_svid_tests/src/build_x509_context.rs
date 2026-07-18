use anyhow::Result;
use spiffe::TrustDomain;
use spiffe::X509Bundle;
use spiffe::X509BundleSet;
use spiffe::X509Context;
use spiffe::X509Svid;

pub fn build_x509_context(
    cert_chain_der: &[u8],
    private_key_der: &[u8],
    trust_domain_name: &str,
    bundle_authorities: &[&[u8]],
) -> Result<X509Context> {
    let svid = X509Svid::parse_from_der(cert_chain_der, private_key_der)?;
    let trust_domain = TrustDomain::new(trust_domain_name)?;
    let bundle = X509Bundle::from_x509_authorities(trust_domain, bundle_authorities)?;
    let mut bundle_set = X509BundleSet::new();

    bundle_set.add_bundle(bundle);

    Ok(X509Context::new(vec![svid], bundle_set))
}

#[cfg(test)]
mod tests {
    use crate::ca_der::CA_DER;
    use crate::leaf_spiffe_example_org_workload_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER;
    use crate::leaf_spiffe_example_org_workload_key_der::LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER;

    use super::build_x509_context;

    fn valid_cert_chain() -> Vec<u8> {
        [LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_DER, CA_DER].concat()
    }

    #[test]
    fn builds_for_valid_inputs() {
        let chain = valid_cert_chain();

        let result = build_x509_context(
            &chain,
            LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
            "example.org",
            &[CA_DER],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn errors_on_malformed_cert_chain() {
        let result = build_x509_context(
            &[0u8; 8],
            LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
            "example.org",
            &[CA_DER],
        );

        assert!(result.is_err());
    }

    #[test]
    fn errors_on_invalid_trust_domain_name() {
        let chain = valid_cert_chain();

        let result = build_x509_context(
            &chain,
            LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
            "INVALID UPPERCASE",
            &[CA_DER],
        );

        assert!(result.is_err());
    }

    #[test]
    fn errors_on_malformed_bundle_authority() {
        let chain = valid_cert_chain();

        let result = build_x509_context(
            &chain,
            LEAF_SPIFFE_EXAMPLE_ORG_WORKLOAD_KEY_DER,
            "example.org",
            &[&[0u8; 8]],
        );

        assert!(result.is_err());
    }
}
