use std::collections::BTreeSet;

use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;

#[test]
fn forms_an_audience_target_without_scopes() {
    let form = TokenTarget {
        audience: TargetAudience::Audience("artifact-store".to_string()),
        scopes: BTreeSet::new(),
    }
    .form_parameters();

    assert_eq!(
        form,
        vec![FormParameter {
            name: "audience",
            value: "artifact-store".to_string(),
        }]
    );
}
