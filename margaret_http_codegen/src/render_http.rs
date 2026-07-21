use margaret_attributes::attribute_index::AttributeIndex;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::http_server::HttpServer;
use crate::render::render;
use crate::render::responder_injects_views;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;
use crate::server_transport_policy::ServerTransportPolicy;

fn merge_websocket_servers(
    mut servers: Vec<HttpServer>,
    websocket_servers: &[String],
) -> Vec<HttpServer> {
    for websocket_server in websocket_servers {
        if !servers
            .iter()
            .any(|server| server.name() == websocket_server)
        {
            servers.push(HttpServer::new(
                websocket_server.clone(),
                ServerTransportPolicy::Negotiable,
            ));
        }
    }

    servers.sort_by_key(|server| server.name().to_string());

    servers
}

pub fn render_http(
    index: &AttributeIndex,
    has_views: bool,
    websocket_servers: &[String],
    middleware_plans: &[MiddlewarePlan],
) -> Result<HttpArtifacts, HttpCodegenError> {
    let table = http_routes(index, middleware_plans)?;

    if !has_views {
        if let Some(route) = table.routes().find(|route| responder_injects_views(route)) {
            return Err(HttpCodegenError::ViewInjectedWithoutViews {
                responder: route.responder_path.to_string(),
            });
        }

        if let Some(plan) = middleware_plans.iter().find(|plan| plan.injects_views()) {
            return Err(HttpCodegenError::MiddlewareViewInjectedWithoutViews {
                middleware: plan.concrete.to_string(),
            });
        }
    }

    let servers = merge_websocket_servers(active_servers(&table), websocket_servers);
    let mut modules = render(&table, &servers, has_views, websocket_servers);

    modules.extend(render_routes(&table, &servers));
    modules.extend(render_forwarders(&table, &servers));

    Ok(HttpArtifacts::new(modules, servers))
}
