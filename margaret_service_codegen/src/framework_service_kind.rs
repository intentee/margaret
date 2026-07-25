use margaret_attributes::canonical_path::CanonicalPath;

pub enum FrameworkServiceKind {
    Service,
    Ticker { interval: CanonicalPath },
}
