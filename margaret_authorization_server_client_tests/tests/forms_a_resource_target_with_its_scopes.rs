use std::collections::BTreeSet;

use url::Url;

use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;

#[test]
fn forms_a_resource_target_with_its_scopes() {
    let form = TokenTarget {
        audience: TargetAudience::Resource(
            Url::parse("https://artifacts.example/").expect("the resource is a url"),
        ),
        scopes: BTreeSet::from([
            "write".parse().expect("the scope is a scope token"),
            "read".parse().expect("the scope is a scope token"),
        ]),
    }
    .form_parameters();

    assert_eq!(
        form,
        vec![
            FormParameter {
                name: "resource",
                value: "https://artifacts.example/".to_string(),
            },
            FormParameter {
                name: "scope",
                value: "read write".to_string(),
            },
        ]
    );
}
