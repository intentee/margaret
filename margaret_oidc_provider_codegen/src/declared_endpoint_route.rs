use margaret_attributes::canonical_path::CanonicalPath;
use margaret_http_codegen::framework_input::FrameworkInput;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::provider_endpoint::ProviderEndpoint;

pub struct DeclaredEndpointRoute {
    pub endpoint: ProviderEndpoint,
    pub input: FrameworkInput,
    pub path: RoutePath,
    pub route: CanonicalPath,
    pub server: String,
}
