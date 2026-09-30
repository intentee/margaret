use std::collections::BTreeMap;

use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;

use crate::unrolled_provider::UnrolledProvider;
use crate::userinfo_grant_of::userinfo_grant_of;

#[test]
fn reports_userinfo_claims_that_cannot_be_serialized() {
    let provider = UnrolledProvider::create();
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance.as_ref());

    assert!(matches!(
        endpoint.answer(&userinfo_grant_of(), &BTreeMap::from([(vec![1_u8], 1_u8)])),
        Err(ProviderError::UserinfoClaimsSerialization(_))
    ));
}
