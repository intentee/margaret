use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum ServiceKind {
    Service,
    Ticker {
        behavior: Option<CanonicalPath>,
        interval: CanonicalPath,
    },
}
