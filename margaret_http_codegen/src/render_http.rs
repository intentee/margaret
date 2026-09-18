use margaret_container::container_bindings::ContainerBindings;
use margaret_server_codegen::http_server::HttpServer;

use crate::http_artifacts::HttpArtifacts;
use crate::http_plan::HttpPlan;
use crate::render::render;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;

#[must_use]
pub fn render_http(
    plan: HttpPlan,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
) -> HttpArtifacts {
    let mut modules = render(&plan.table, servers, plan.has_views, bindings);

    modules.extend(render_routes(&plan.table, servers));
    modules.extend(render_forwarders(&plan.table, servers));

    HttpArtifacts::new(modules, plan.retained_roots)
}
