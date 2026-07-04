use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct InterceptorReference {
    pub(crate) injects_routes: bool,
    pub(crate) interceptor: CanonicalPath,
}
