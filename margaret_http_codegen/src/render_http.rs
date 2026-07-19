use margaret_attributes::attribute_index::AttributeIndex;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::middleware_plans::middleware_plans;
use crate::render::render;
use crate::render::responder_injects_views;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;

pub fn render_http(
    index: &AttributeIndex,
    has_views: bool,
) -> Result<HttpArtifacts, HttpCodegenError> {
    let middleware_plans = middleware_plans(index)?;
    let table = http_routes(index, &middleware_plans)?;

    if !has_views
        && let Some(route) = table.routes().find(|route| responder_injects_views(route))
    {
        return Err(HttpCodegenError::ViewInjectedWithoutViews {
            responder: route.responder_path.to_string(),
        });
    }

    let servers = active_servers(&table);
    let mut modules = render(&table, &servers, &middleware_plans, has_views);

    modules.extend(render_routes(&table, &servers));
    modules.extend(render_forwarders(&table, &servers));

    Ok(HttpArtifacts::new(modules, servers))
}
