use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
use margaret_sessions_codegen::sign_out_endpoint_module::SignOutEndpointModule;

pub struct SessionSignOutRoute {
    pub handler: SignOutEndpointModule,
    pub landing: RouteUrlInput,
    pub route: CanonicalPath,
}
