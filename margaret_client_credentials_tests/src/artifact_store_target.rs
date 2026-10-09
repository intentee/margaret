use std::collections::BTreeSet;

use serde_json::Value;
use url::Url;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_oauth_vocabulary::scope::Scope;

/// # Panics
///
/// Panics when the fixture resource or scope is rejected.
#[must_use]
pub fn artifact_store_target() -> TokenTarget {
    TokenTarget {
        audience: TargetAudience::Resource(
            Url::parse("https://artifacts.example/").expect("the fixture resource is a url"),
        ),
        scopes: BTreeSet::from([serde_json::from_value::<Scope>(Value::String(
            "artifacts:write".to_string(),
        ))
        .expect("the fixture scope is a scope token")]),
    }
}
