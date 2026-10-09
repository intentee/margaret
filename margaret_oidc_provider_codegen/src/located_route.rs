use margaret_route_parameter_codegen::route_path::RoutePath;

pub(crate) struct LocatedRoute<'plan> {
    pub(crate) path: &'plan RoutePath,
    pub(crate) server: &'plan str,
}
