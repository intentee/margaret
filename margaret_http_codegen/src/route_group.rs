use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_route::HttpRoute;

pub(crate) struct RouteGroup {
    methods: Vec<HttpRoute>,
    path: RoutePath,
}

impl RouteGroup {
    pub(crate) fn new(path: RoutePath) -> Self {
        Self {
            methods: Vec::new(),
            path,
        }
    }

    pub(crate) fn method_routes(&self) -> impl Iterator<Item = &HttpRoute> {
        self.methods.iter()
    }

    pub(crate) fn path(&self) -> &RoutePath {
        &self.path
    }

    pub(crate) fn take_method(
        &mut self,
        method: http::Method,
        route: HttpRoute,
    ) -> Option<HttpRoute> {
        match self
            .methods
            .iter()
            .position(|existing| existing.method == method)
        {
            Some(position) => Some(std::mem::replace(&mut self.methods[position], route)),
            None => {
                self.methods.push(route);

                None
            }
        }
    }
}
