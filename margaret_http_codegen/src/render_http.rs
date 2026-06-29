use margaret_attributes::attribute_index::AttributeIndex;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_routes::http_routes;
use crate::middleware_bindings::middleware_bindings;
use crate::render::render;

pub fn render_http(index: &AttributeIndex) -> Result<String, HttpCodegenError> {
    let bindings = middleware_bindings(index)?;
    let routes = http_routes(index, &bindings)?;

    Ok(render(&routes))
}
