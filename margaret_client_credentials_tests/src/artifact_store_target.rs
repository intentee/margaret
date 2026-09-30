use std::collections::BTreeSet;

use url::Url;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;

/// # Panics
///
/// Panics when the fixture resource or scope is rejected.
#[must_use]
pub fn artifact_store_target() -> TokenTarget {
    TokenTarget {
        audience: TargetAudience::Resource(
            Url::parse("https://artifacts.example/").expect("the fixture resource is a url"),
        ),
        scopes: BTreeSet::from(["artifacts:write"
            .parse()
            .expect("the fixture scope is a scope token")]),
    }
}
