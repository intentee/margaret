use std::collections::BTreeMap;

use margaret_claims_merge::claims_merge_error::ClaimsMergeError;
use margaret_handler_error::handler_error::HandlerError;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::fixed_userinfo_claims::FixedUserinfoClaims;
use margaret_oidc_provider_tests::userinfo_handled::userinfo_handled;

#[tokio::test]
async fn reports_userinfo_claims_that_cannot_be_serialized() {
    let Err(HandlerError::Consumer { source }) = userinfo_handled(
        &END_USER_SUBJECT.to_string(),
        FixedUserinfoClaims {
            claims: BTreeMap::from([(vec![1_u8], 1_u8)]),
        },
    )
    .await
    else {
        panic!("claims that cannot be serialized are reported");
    };

    assert!(matches!(
        source.downcast_ref::<ProviderError>(),
        Some(ProviderError::UserinfoClaims(
            ClaimsMergeError::Serialization(_)
        ))
    ));
}
