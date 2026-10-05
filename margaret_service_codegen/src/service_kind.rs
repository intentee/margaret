use margaret_attributes::canonical_path::CanonicalPath;

use crate::first_tick::FirstTick;

pub(crate) enum ServiceKind {
    Service,
    Ticker {
        behavior: Option<CanonicalPath>,
        first_tick: FirstTick,
        interval: CanonicalPath,
    },
}
