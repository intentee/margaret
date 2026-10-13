use std::time::Duration;

use http::HeaderName;

pub const AUDIT_INTERVAL: Duration = Duration::from_mins(1);

pub const COURIER_NAME: HeaderName = HeaderName::from_static("x-courier-name");

pub const LIBRARIAN_BADGE: HeaderName = HeaderName::from_static("x-librarian-badge");

pub const PATRON_CARD: HeaderName = HeaderName::from_static("x-patron-card");

pub const VISITOR_NAME: HeaderName = HeaderName::from_static("x-visitor-name");
