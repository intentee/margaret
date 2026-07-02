use margaret_attributes::attribute_index::AttributeIndex;

use crate::active_servers::active_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::middleware_bindings::middleware_bindings;
use crate::render::render;

pub fn render_http(index: &AttributeIndex) -> Result<HttpArtifacts, HttpCodegenError> {
    let bindings = middleware_bindings(index)?;
    let routes = http_routes(index, &bindings)?;
    let servers = active_servers(&routes);
    let source = render(&routes, &servers);

    Ok(HttpArtifacts::new(source, servers))
}
