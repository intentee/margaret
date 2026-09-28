use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn locates_the_discovery_document_at_the_issuer_root() {
    let issuer = "https://server.example.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert_eq!(
        oidc_discovery_url(&issuer).as_str(),
        "https://server.example.com/.well-known/openid-configuration"
    );
}
