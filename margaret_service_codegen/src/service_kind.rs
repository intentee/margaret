use syn::Path;

pub(crate) enum ServiceKind {
    Service,
    Ticker {
        behavior: Option<Path>,
        interval: Path,
    },
}
