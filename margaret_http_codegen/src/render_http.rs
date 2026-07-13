use margaret_attributes::attribute_index::AttributeIndex;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::interceptor_plans::interceptor_plans;
use crate::interceptor_references::interceptor_references;
use crate::middleware_plans::middleware_plans;
use crate::render::render;
use crate::render_forwarders::render_forwarders;
use crate::render_routes::render_routes;

pub fn render_http(index: &AttributeIndex) -> Result<HttpArtifacts, HttpCodegenError> {
    let middleware_plans = middleware_plans(index)?;
    let interceptor_plans = interceptor_plans(index, index.trait_resolution())?;
    let interceptors = interceptor_references(&interceptor_plans)?;
    let routes = http_routes(index, &middleware_plans, &interceptors)?;
    let servers = active_servers(&routes);
    let mut modules = render(
        &routes,
        &servers,
        &interceptor_plans,
        &middleware_plans,
        index,
    );

    modules.extend(render_routes(&routes, &servers));
    modules.extend(render_forwarders(&routes, &servers));

    Ok(HttpArtifacts::new(modules, servers))
}
