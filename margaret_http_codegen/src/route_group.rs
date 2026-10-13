use std::collections::BTreeMap;

use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_route::HttpRoute;

pub(crate) struct RouteGroup {
    methods: BTreeMap<RouteMethod, HttpRoute>,
    path: RoutePath,
}

impl RouteGroup {
    pub(crate) fn new(path: RoutePath) -> Self {
        Self {
            methods: BTreeMap::new(),
            path,
        }
    }

    pub(crate) fn method_routes(&self) -> impl Iterator<Item = &HttpRoute> {
        self.methods.values()
    }

    pub(crate) fn path(&self) -> &RoutePath {
        &self.path
    }

    pub(crate) fn take_method(
        &mut self,
        method: RouteMethod,
        route: HttpRoute,
    ) -> Option<HttpRoute> {
        self.methods.insert(method, route)
    }
}
