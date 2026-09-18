use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_root::binding_root;
use margaret_server_codegen::server_contribution::ServerContribution;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route_table::HttpRouteTable;
use crate::http_routes::http_routes;
use crate::http_server_contributions::http_server_contributions;

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

pub struct HttpPlan {
    pub(crate) has_views: bool,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    server_contributions: Vec<ServerContribution>,
    pub(crate) table: HttpRouteTable,
}

impl HttpPlan {
    /// # Errors
    ///
    /// Returns `HttpCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        has_views: bool,
        middleware_plans: &[MiddlewarePlan],
        registries: &BindingRegistries,
        bindings: &ContainerBindings,
    ) -> Result<Self, HttpCodegenError> {
        let table = http_routes(index, middleware_plans, registries, bindings)?;
        let retained_roots = retained_roots(&table);
        let server_contributions = http_server_contributions(&table);

        Ok(Self {
            has_views,
            retained_roots,
            server_contributions,
            table,
        })
    }

    #[must_use]
    pub fn server_contributions(&self) -> &[ServerContribution] {
        &self.server_contributions
    }
}
