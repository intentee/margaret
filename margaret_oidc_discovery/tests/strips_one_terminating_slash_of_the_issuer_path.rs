use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn strips_one_terminating_slash_of_the_issuer_path() {
    let issuer = "https://example.com/issuer1/"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert_eq!(
        oidc_discovery_url(&issuer).as_str(),
        "https://example.com/issuer1/.well-known/openid-configuration"
    );
}
