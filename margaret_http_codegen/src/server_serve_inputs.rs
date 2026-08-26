use std::collections::BTreeMap;

use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_serve_inputs::binding_serve_inputs;
use margaret_serve_input_codegen::serve_input::ServeInput;

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
) -> Result<Vec<ServeInput>, HttpCodegenError> {
    let mut collected = Vec::new();

    for route in routes {
        collected.extend_from_slice(
            &bindings
                .provider_serve_inputs(&route.responder_path)?
                .inputs,
        );

        for argument in &route.arguments {
            collected.extend(binding_serve_inputs(&argument.binding, bindings)?);
        }

        for layer in &route.layers {
            collected.extend_from_slice(&bindings.provider_serve_inputs(&layer.concrete)?.inputs);
        }
    }

    Ok(collected)
}

pub(crate) fn server_serve_inputs(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
    websocket_server_serve_inputs: &BTreeMap<String, Vec<ServeInput>>,
) -> Result<BTreeMap<String, Vec<ServeInput>>, HttpCodegenError> {
    servers
        .iter()
        .map(|server| {
            let routes = server_routes(table, server.name());
            let mut collected = responder_and_binder_arguments(&routes, bindings)?;

            if let Some(websocket_inputs) = websocket_server_serve_inputs.get(server.name()) {
                collected.extend_from_slice(websocket_inputs);
            }

            Ok((
                server.name().to_string(),
                bindings.serve_input_union(&collected)?,
            ))
        })
        .collect()
}
