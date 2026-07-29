use rustls::CertificateError;
use rustls::Error;

use margaret_spiffe_svid_tests::peer_identity_rejection::peer_identity_rejection;

#[test]
fn peer_identity_rejection_reports_none_for_an_unrelated_error() {
    let unrelated = Error::InvalidCertificate(CertificateError::Expired);

    assert!(peer_identity_rejection(&unrelated).is_none());
}
