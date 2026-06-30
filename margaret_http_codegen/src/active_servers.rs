use std::collections::HashSet;

use crate::declared_server::DeclaredServer;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;

pub(crate) fn active_servers(
    declared: &[DeclaredServer],
    routes: &[HttpRoute],
) -> Result<Vec<HttpServer>, HttpCodegenError> {
    let used: HashSet<&str> = routes.iter().map(|route| route.server.as_str()).collect();
    let mut servers: Vec<HttpServer> = Vec::new();

    for declared_server in declared {
        if !used.contains(declared_server.name()) {
            return Err(HttpCodegenError::DeclaredHttpServerWithoutRoutes {
                server: declared_server.name().to_string(),
            });
        }

        servers.push(HttpServer::new(declared_server.name().to_string()));
    }

    Ok(servers)
}
