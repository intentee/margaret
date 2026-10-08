use std::collections::BTreeMap;

use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_oidc_provider_tests::unserved_provider::UnservedProvider;
use margaret_oidc_provider_tests::userinfo_grant_of::userinfo_grant_of;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;

#[tokio::test]
async fn reports_userinfo_claims_that_cannot_be_serialized() {
    let provider = UnservedProvider::create().await;
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance);

    assert!(matches!(
        endpoint.answer(&userinfo_grant_of(), &BTreeMap::from([(vec![1_u8], 1_u8)])),
        Err(ProviderError::UserinfoClaims(
            ClaimsMergeError::Serialization(_)
        ))
    ));
}
