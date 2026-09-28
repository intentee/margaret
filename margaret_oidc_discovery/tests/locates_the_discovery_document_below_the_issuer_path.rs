use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn locates_the_discovery_document_below_the_issuer_path() {
    let issuer = "https://example.com/issuer1"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert_eq!(
        oidc_discovery_url(&issuer).as_str(),
        "https://example.com/issuer1/.well-known/openid-configuration"
    );
}
