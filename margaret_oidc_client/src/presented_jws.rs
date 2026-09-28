use std::sync::Arc;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::jws_rejection::JwsRejection;

pub(crate) enum PresentedJws<'request> {
    Parsed(CompactJws<'request>),
    Unparseable(Arc<JwsRejection>),
}
