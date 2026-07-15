use margaret_attributes::attribute_index::AttributeIndex;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::middleware_plans::middleware_plans;
use crate::render::render;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;

pub fn render_http(index: &AttributeIndex) -> Result<HttpArtifacts, HttpCodegenError> {
    let middleware_plans = middleware_plans(index)?;
    let table = http_routes(index, &middleware_plans)?;
    let servers = active_servers(&table);
    let mut modules = render(&table, &servers, &middleware_plans, index);

    modules.extend(render_routes(&table, &servers));
    modules.extend(render_forwarders(&table, &servers));

    Ok(HttpArtifacts::new(modules, servers))
}
