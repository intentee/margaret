use url::Url;

use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::scope_list_parsing::ScopeListParsing;

#[test]
fn forms_a_resource_target_with_its_scopes() {
    let ScopeListParsing::Accepted(ScopeList { scopes }) = ScopeList::parse("write read") else {
        panic!("the scopes are scope tokens");
    };
    let form = TokenTarget {
        audience: TargetAudience::Resource(
            Url::parse("https://artifacts.example/").expect("the resource is a url"),
        ),
        scopes,
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
