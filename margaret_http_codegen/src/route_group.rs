use std::collections::BTreeMap;

use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_route::HttpRoute;

pub(crate) struct RouteGroup {
    methods: BTreeMap<String, HttpRoute>,
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

    pub(crate) fn take_method(&mut self, method: String, route: HttpRoute) -> Option<HttpRoute> {
        self.methods.insert(method, route)
    }
}
