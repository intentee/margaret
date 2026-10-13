use serde_json::json;

use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn parses_a_scope_list_delimited_by_spaces() {
    let ScopeList { scopes } = serde_json::from_value(json!("openid profile"))
        .expect("the scope list is space delimited scope tokens");

    assert_eq!(
        scopes.iter().map(Scope::as_str).collect::<Vec<&str>>(),
        vec!["openid", "profile"]
    );
}
