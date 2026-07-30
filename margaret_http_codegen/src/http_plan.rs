use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::active_servers::active_servers;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route_table::HttpRouteTable;
use crate::http_routes::http_routes;
use crate::http_server::HttpServer;
use crate::server_console_arguments::server_console_arguments;
use crate::server_transport_policy::ServerTransportPolicy;

pub struct HttpPlan {
    pub(crate) has_views: bool,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    pub(crate) servers: Vec<HttpServer>,
    pub(crate) table: HttpRouteTable,
    pub(crate) websocket_servers: Vec<String>,
}

impl HttpPlan {
    /// # Errors
    ///
    /// Returns `HttpCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        has_views: bool,
        websocket_servers: &[String],
        middleware_plans: &[MiddlewarePlan],
        bindings: &ContainerBindings,
        websocket_server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
        registries: &BindingRegistries,
    ) -> Result<Self, HttpCodegenError> {
        let table = http_routes(index, middleware_plans, registries)?;
        let servers = merge_websocket_servers(active_servers(&table), websocket_servers);
        let server_console_arguments =
            server_console_arguments(&table, &servers, bindings, websocket_server_arguments)?;
        let retained_roots = retained_roots(&table);

        Ok(Self {
            has_views,
            retained_roots,
            server_console_arguments,
            servers,
            table,
            websocket_servers: websocket_servers.to_vec(),
        })
    }
}

fn binding_root(binding: &RequestBinding) -> Option<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => Some(&application.concrete),
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => Some(binder_provider),
        RequestBinding::Injectable { dependency } => Some(&dependency.concrete),
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => None,
    }
}

fn retained_roots(table: &HttpRouteTable) -> Vec<CanonicalPath> {
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
            servers.push(HttpServer::new(
                websocket_server.clone(),
                ServerTransportPolicy::Negotiable,
            ));
        }
    }

    servers.sort_by_key(|server| server.name().to_string());

    servers
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
        let binding = RequestBinding::BoundRouteParameter {
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
