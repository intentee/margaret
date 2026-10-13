use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope::Scope;

/// # Panics
///
/// Panics when a fixture scope is not a scope token.
#[must_use]
pub fn fixture_scopes(scopes: &[&str]) -> BTreeSet<Scope> {
    scopes
        .iter()
        .map(|scope| {
            serde_json::from_value::<Scope>(serde_json::Value::String((*scope).to_string()))
                .expect("the fixture scope is a scope token")
        })
        .collect()
}
