use margaret_route_parameter_codegen::route_path::RoutePath;

pub struct BindingSite<'site> {
    pub route_path: &'site RoutePath,
    pub server: &'site str,
    pub subject: &'site str,
}
