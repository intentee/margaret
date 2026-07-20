use std::sync::Arc;

use url::Url;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

use crate::static_endpoint::StaticEndpoint;

#[must_use]
pub fn unreachable_endpoint() -> Arc<dyn ProvidesEndpoint> {
    Arc::new(StaticEndpoint::new(
        Url::parse("https://127.0.0.1:1").expect("the unreachable origin parses"),
    ))
}
