use std::num::NonZeroU64;

use margaret_request_binding_codegen::content_binding::ContentBinding;

pub(crate) enum RouteContent {
    Read {
        binding: ContentBinding,
        limit: NonZeroU64,
    },
    Unread,
}
