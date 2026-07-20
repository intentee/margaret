use margaret_attributes::attribute_index::AttributeIndex;
use margaret_codegen_tokens::synthetic_route::SyntheticRoute;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_routes::http_routes;
use crate::middleware_plans::middleware_plans;
use crate::render::render;
use crate::render::responder_injects_views;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;
use crate::route_handler::RouteHandler;
use crate::route_path::RoutePath;

fn fold_synthetic_routes(
    table: &mut HttpRouteTable,
    synthetic_routes: &[SyntheticRoute],
) -> Result<(), HttpCodegenError> {
    for SyntheticRoute {
        handler,
        label,
        path,
        server,
    } in synthetic_routes
    {
        table.insert(
            RoutePath::parse(path),
            HttpRoute {
                arguments: Vec::new(),
                handler: RouteHandler::Synthetic {
                    handler: handler.clone(),
                    label: label.clone(),
                },
                layers: Vec::new(),
                method: "GET".to_owned(),
                name: None,
                server: server.clone(),
            },
        )?;
    }

    Ok(())
}

pub fn render_http(
    index: &AttributeIndex,
    has_views: bool,
    synthetic_routes: &[SyntheticRoute],
) -> Result<HttpArtifacts, HttpCodegenError> {
    let middleware_plans = middleware_plans(index)?;
    let mut table = http_routes(index, &middleware_plans)?;

    fold_synthetic_routes(&mut table, synthetic_routes)?;

    if !has_views
        && let Some(route) = table.routes().find(|route| responder_injects_views(route))
    {
        return Err(HttpCodegenError::ViewInjectedWithoutViews {
            responder: route.handler.describe(),
        });
    }

    let servers = active_servers(&table);
    let mut modules = render(&table, &servers, &middleware_plans, has_views);

    modules.extend(render_routes(&table, &servers));
    modules.extend(render_forwarders(&table, &servers));

    Ok(HttpArtifacts::new(modules, servers))
}
