use margaret_attributes::canonical_path::CanonicalPath;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;

pub struct RouteLocation<'plan> {
    pub method: RouteMethod,
    pub path: &'plan RoutePath,
    pub responder_path: &'plan CanonicalPath,
    pub server: &'plan str,
}
