use std::collections::BTreeSet;

use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;

pub(crate) fn active_servers(routes: &[HttpRoute]) -> Vec<HttpServer> {
    routes
        .iter()
        .map(|route| route.server.as_str())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|name| HttpServer::new(name.to_string()))
        .collect()
}
