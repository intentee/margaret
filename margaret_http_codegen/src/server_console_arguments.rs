use std::collections::BTreeMap;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_console_arguments::binding_console_arguments;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::route_group::RouteGroup;

fn server_routes<'table>(table: &'table HttpRouteTable, server: &str) -> Vec<&'table HttpRoute> {
    table
        .route_groups(server)
        .flat_map(RouteGroup::method_routes)
        .collect()
}

fn responder_and_binder_arguments(
    routes: &[&HttpRoute],
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, HttpCodegenError> {
    let mut collected = Vec::new();

    for route in routes {
        collected.extend_from_slice(&bindings.console_arguments(&route.responder_path)?.arguments);

        for argument in &route.arguments {
            collected.extend(binding_console_arguments(&argument.binding, bindings)?);
        }

        for layer in &route.layers {
            collected.extend_from_slice(&bindings.console_arguments(&layer.concrete)?.arguments);
        }
    }

    Ok(collected)
}

pub(crate) fn server_console_arguments(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
    websocket_server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
) -> Result<BTreeMap<String, Vec<ConsoleArgument>>, HttpCodegenError> {
    servers
        .iter()
        .map(|server| {
            let routes = server_routes(table, server.name());
            let mut collected = responder_and_binder_arguments(&routes, bindings)?;

            if let Some(websocket_arguments) = websocket_server_arguments.get(server.name()) {
                collected.extend_from_slice(websocket_arguments);
            }

            Ok((
                server.name().to_string(),
                bindings.console_union(&collected)?,
            ))
        })
        .collect()
}
