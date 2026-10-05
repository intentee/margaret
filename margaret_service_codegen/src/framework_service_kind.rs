use margaret_attributes::canonical_path::CanonicalPath;

use crate::first_tick::FirstTick;

pub enum FrameworkServiceKind {
    Service,
    Ticker {
        first_tick: FirstTick,
        interval: CanonicalPath,
    },
}
