use serde_json::json;

use margaret_oauth_vocabulary::scope::Scope;

#[test]
fn refuses_a_scope_that_is_not_a_json_string() {
    assert!(serde_json::from_value::<Scope>(json!(7)).is_err());
    assert!(serde_json::from_value::<Scope>(json!("")).is_err());
}
