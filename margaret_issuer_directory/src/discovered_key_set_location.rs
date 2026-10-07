use url::Url;

use crate::discovery_failure::DiscoveryFailure;

pub(crate) enum DiscoveredKeySetLocation {
    Cancelled,
    Failed(DiscoveryFailure),
    Located(Url),
}
