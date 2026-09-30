use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn renders_a_scope_list_delimited_by_spaces() {
    assert_eq!(
        ScopeList {
            scopes: BTreeSet::from([
                "profile".parse().expect("the scope is a scope token"),
                "openid".parse().expect("the scope is a scope token"),
            ]),
        }
        .to_string(),
        "openid profile"
    );
}
