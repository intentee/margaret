use margaret_attributes::canonical_path::CanonicalPath;
use margaret_http_codegen::framework_input::FrameworkInput;
use margaret_route_parameter_codegen::route_path::RoutePath;

pub struct DeclaredConsentRoute {
    pub input: FrameworkInput,
    pub path: RoutePath,
    pub route: CanonicalPath,
    pub server: String,
    pub view: CanonicalPath,
}
