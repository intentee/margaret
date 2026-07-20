use margaret_attributes::canonical_path::CanonicalPath;

pub struct RouteParameterBinder {
    pub field: String,
    pub provider: CanonicalPath,
}
