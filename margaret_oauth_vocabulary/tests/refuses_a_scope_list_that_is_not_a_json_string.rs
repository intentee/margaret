use serde_json::json;

use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn refuses_a_scope_list_that_is_not_a_json_string() {
    assert!(serde_json::from_value::<ScopeList>(json!(["openid"])).is_err());
}
