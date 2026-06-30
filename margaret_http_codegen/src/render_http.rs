use margaret_attributes::attribute_index::AttributeIndex;

use crate::active_servers::active_servers;
use crate::authenticated_actor_store::authenticated_actor_store;
use crate::declared_servers::declared_servers;
use crate::http_artifacts::HttpArtifacts;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::middleware_bindings::middleware_bindings;
use crate::render::render;
use crate::site_action_gates::site_action_gates;

pub fn render_http(index: &AttributeIndex) -> Result<HttpArtifacts, HttpCodegenError> {
    let bindings = middleware_bindings(index)?;
    let store = authenticated_actor_store(index)?;
    let site_gates = site_action_gates(index)?;
    let declared = declared_servers(index)?;
    let routes = http_routes(index, &bindings, &site_gates, store.is_some(), &declared)?;
    let servers = active_servers(&declared, &routes)?;
    let source = render(&routes, &servers);

    Ok(HttpArtifacts::new(source, servers))
}
