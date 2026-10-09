use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_oidc_provider_tests::unserved_provider::UnservedProvider;
use margaret_oidc_provider_tests::userinfo_grant_of::userinfo_grant_of;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;

#[tokio::test]
async fn refuses_userinfo_claims_that_are_not_an_object() {
    let provider = UnservedProvider::create();
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance);

    assert!(matches!(
        endpoint.answer(&userinfo_grant_of(), &"Ada"),
        Err(ProviderError::UserinfoClaims(ClaimsMergeError::NotAnObject))
    ));
}
