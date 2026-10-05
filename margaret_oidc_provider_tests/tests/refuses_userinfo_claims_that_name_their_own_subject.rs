use serde_json::json;

use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_oidc_provider_tests::unserved_provider::UnservedProvider;
use margaret_oidc_provider_tests::userinfo_grant_of::userinfo_grant_of;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;

#[test]
fn refuses_userinfo_claims_that_name_their_own_subject() {
    let provider = UnservedProvider::create();
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance.as_ref());

    assert!(matches!(
        endpoint.answer(&userinfo_grant_of(), &json!({"sub": "someone"})),
        Err(ProviderError::UserinfoClaims(ClaimsMergeError::Colliding { member })) if member == "sub"
    ));
}
