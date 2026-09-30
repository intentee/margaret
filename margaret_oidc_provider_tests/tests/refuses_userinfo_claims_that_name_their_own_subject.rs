use serde_json::json;

use margaret_oidc_provider::userinfo_answer::UserinfoAnswer;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;

use crate::unrolled_provider::UnrolledProvider;
use crate::userinfo_grant_of::userinfo_grant_of;

#[test]
fn refuses_userinfo_claims_that_name_their_own_subject() {
    let provider = UnrolledProvider::create();
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance.as_ref());

    assert!(matches!(
        endpoint.answer(&userinfo_grant_of(), &json!({"sub": "someone"})),
        Ok(UserinfoAnswer::CollidingSubject)
    ));
}
