use serde_json::json;

use margaret_oauth_vocabulary::scope::Scope;

#[test]
fn reads_a_scope_from_a_json_string() {
    let scope: Scope = serde_json::from_value(json!("openid")).expect("the scope is a scope token");

    assert_eq!(scope.as_str(), "openid");
    assert_eq!(
        serde_json::to_value(&scope).expect("the scope serializes"),
        json!("openid")
    );
}
