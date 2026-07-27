use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::http_server::HttpServer;
use crate::render::render;
use crate::render::server_console_arguments;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;
use crate::server_transport_policy::ServerTransportPolicy;

fn binding_root(binding: &RequestBinding) -> Option<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => Some(&application.concrete),
        RequestBinding::Bound {
            binder_provider, ..
        } => Some(binder_provider),
        RequestBinding::Injectable { dependency } => Some(&dependency.concrete),
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::Raw { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => None,
    }
}

fn retained_roots(table: &crate::http_route_table::HttpRouteTable) -> Vec<CanonicalPath> {
    let mut roots = std::collections::BTreeSet::new();

    for route in table.routes() {
        roots.insert(route.responder_path.clone());

        for layer in &route.layers {
            roots.insert(layer.concrete.clone());
        }

        for parameter in &route.arguments {
            if let Some(root) = binding_root(&parameter.binding) {
                roots.insert(root.clone());
            }
        }
    }

    roots.into_iter().collect()
}
fn merge_websocket_servers(
    mut servers: Vec<HttpServer>,
    websocket_servers: &[String],
) -> Vec<HttpServer> {
    for websocket_server in websocket_servers {
        if !servers
            .iter()
            .any(|server| server.name() == websocket_server)
        {
            servers.push(
                HttpServer::new(websocket_server.clone(), ServerTransportPolicy::Negotiable)
                    .with_async_routes(),
            );
        }
    }

    servers = servers
        .into_iter()
        .map(|server| {
            if websocket_servers
                .iter()
                .any(|websocket_server| websocket_server == server.name())
            {
                server.with_async_routes()
            } else {
                server
            }
        })
        .collect();
    servers.sort_by_key(|server| server.name().to_string());

    servers
}

pub fn render_http(
    index: &AttributeIndex,
    has_views: bool,
    websocket_servers: &[String],
    middleware_plans: &[MiddlewarePlan],
    bindings: &ContainerBindings,
    websocket_server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    registries: &BindingRegistries,
) -> Result<HttpArtifacts, HttpCodegenError> {
    let table = http_routes(index, middleware_plans, registries)?;
    let servers = merge_websocket_servers(active_servers(&table), websocket_servers);
    let server_arguments =
        server_console_arguments(&table, &servers, bindings, websocket_server_arguments)?;
    let mut modules = render(&table, &servers, has_views, websocket_servers, bindings);

    modules.extend(render_routes(&table, &servers));
    modules.extend(render_forwarders(&table, &servers));

    Ok(HttpArtifacts::new(
        modules,
        servers,
        server_arguments,
        retained_roots(&table),
    ))
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_container::injected_dependency::InjectedDependency;
    use margaret_request_binding_codegen::request_binding::RequestBinding;

    use super::binding_root;

    #[test]
    fn identifies_a_route_parameter_binder_as_a_retained_root() {
        let binder_provider =
            CanonicalPath::new(vec!["crate".to_string(), "UserBinder".to_string()]);
        let binding = RequestBinding::Bound {
            binder_field: "user_binder".to_string(),
            binder_provider: binder_provider.clone(),
            path_key: "user".to_string(),
        };

        assert_eq!(binding_root(&binding), Some(&binder_provider));
    }

    #[test]
    fn identifies_an_injected_dependency_as_a_retained_root() {
        let concrete = CanonicalPath::new(vec!["crate".to_string(), "Store".to_string()]);
        let binding = RequestBinding::Injectable {
            dependency: InjectedDependency {
                concrete: concrete.clone(),
                field: "store".to_string(),
            },
        };

        assert_eq!(binding_root(&binding), Some(&concrete));
    }
}
