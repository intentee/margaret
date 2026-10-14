use margaret_container::container_bindings::ContainerBindings;

use crate::http_artifacts::HttpArtifacts;
use crate::http_plan::HttpPlan;
use crate::render::render;
use crate::render_routes::render_routes;
use crate::server_layouts::server_layouts;

#[must_use]
pub fn render_http(plan: HttpPlan, bindings: &ContainerBindings) -> HttpArtifacts {
    let layouts = server_layouts(&plan.table, &plan.servers);
    let mut modules = render(
        &plan.table,
        &layouts,
        plan.has_views,
        &plan.websocket_paths,
        bindings,
    );

    modules.extend(render_routes(&plan.table, &layouts, &plan.websocket_paths));

    HttpArtifacts {
        retained_roots: plan.retained_roots,
        modules,
        servers: plan.servers,
    }
}
