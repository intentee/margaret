use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum ResponderOutput {
    Intercepted {
        interceptor: CanonicalPath,
        injects_routes: bool,
    },
    Plain,
}
