use std::collections::BTreeSet;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;

#[test]
fn forms_nothing_for_an_unspecified_target_without_scopes() {
    assert!(
        TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        }
        .form_parameters()
        .is_empty()
    );
}
