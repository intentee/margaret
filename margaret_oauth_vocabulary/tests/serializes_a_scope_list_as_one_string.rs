use std::collections::BTreeSet;

use serde_json::json;

use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn serializes_a_scope_list_as_one_string() {
    assert_eq!(
        serde_json::to_value(ScopeList {
            scopes: BTreeSet::from([Scope::openid()]),
        })
        .expect("the scope list serializes"),
        json!("openid")
    );
}
