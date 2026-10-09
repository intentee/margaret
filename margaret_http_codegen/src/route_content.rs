use std::num::NonZeroU64;

use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

pub(crate) enum RouteContent<TBinding> {
    Read {
        binding: TBinding,
        limit: NonZeroU64,
        method: ContentMethod,
    },
    Unread {
        method: RouteMethod,
    },
}
