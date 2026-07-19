use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct RouteParameterBinder {
    pub(crate) field: String,
    pub(crate) provider: CanonicalPath,
}
