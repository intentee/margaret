use margaret_container::container_bindings::ContainerBindings;

use crate::http_artifacts::HttpArtifacts;
use crate::http_plan::HttpPlan;
use crate::render::render;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;

#[must_use]
pub fn render_http(plan: HttpPlan, bindings: &ContainerBindings) -> HttpArtifacts {
    let mut modules = render(
        &plan.table,
        &plan.servers,
        plan.has_views,
        &plan.websocket_servers,
        bindings,
    );

    modules.extend(render_routes(&plan.table, &plan.servers));
    modules.extend(render_forwarders(&plan.table, &plan.servers));

    HttpArtifacts::new(
        modules,
        plan.servers,
        plan.server_serve_inputs,
        plan.retained_roots,
    )
}
