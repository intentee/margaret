use rcgen::SanType;
use rcgen::string::Ia5String;
use spiffe::spiffe_id::SpiffeIdError;

use margaret_peer_identity::peer_identity_error::PeerIdentityError;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;
use margaret_peer_identity_tests::self_signed_certificate_der::self_signed_certificate_der;

#[test]
fn spiffe_id_from_cert_rejects_a_uri_with_a_port() {
    let certificate_der = self_signed_certificate_der(vec![SanType::URI(
        Ia5String::try_from("spiffe://example.org:8080/workload")
            .expect("the URI is a valid IA5 string"),
    )]);

    assert!(matches!(
        spiffe_id_from_cert(&certificate_der),
        Err(PeerIdentityError::SpiffeId {
            source: SpiffeIdError::BadTrustDomainChar
        })
    ));
}
