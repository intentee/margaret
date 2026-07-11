use anyhow::Result;
use anyhow::anyhow;
use rustls::pki_types::CertificateDer;
use spiffe::X509Context;
use spiffe::X509Svid;

use crate::ca_bundle::CaBundle;

pub fn extract_ca_bundle(
    default_svid: &X509Svid,
    x509_context: &X509Context,
) -> Result<CaBundle<'static>> {
    let trust_domain = default_svid.spiffe_id().trust_domain();
    let bundle = x509_context
        .bundle_set()
        .get_bundle(trust_domain)
        .ok_or_else(|| anyhow!("Unable to get CA bundle for trust domain '{trust_domain}'"))?;

    let ca_certs: Vec<CertificateDer> = bundle
        .authorities()
        .iter()
        .map(|certificate_authority| CertificateDer::from(certificate_authority.as_ref().to_vec()))
        .collect();

    Ok(CaBundle { ca_certs })
}
