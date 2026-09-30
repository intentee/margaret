use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;

use crate::request_authorized_by::request_authorized_by;
use crate::unrolled_provider::UnrolledProvider;

#[test]
fn reports_userinfo_before_the_first_roll() {
    let provider = UnrolledProvider::create();
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance.as_ref());

    assert!(matches!(
        endpoint.authenticate(&request_authorized_by("Bearer a.b.c")),
        Err(ProviderError::Signing(_))
    ));
}
