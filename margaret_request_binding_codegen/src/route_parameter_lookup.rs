use margaret_attributes::canonical_path::CanonicalPath;

pub enum RouteParameterLookup {
    Binder,
    PrimaryKey { loaded: CanonicalPath },
}
