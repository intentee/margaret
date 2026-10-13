use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

pub struct SignInCallback {
    pub landing: RouteUrlInput,
    pub route: CanonicalPath,
    pub url: RouteUrlInput,
}
