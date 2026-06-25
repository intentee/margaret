use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

use crate::http_codegen_error::HttpCodegenError;
use crate::render_http::render_http;

pub fn generate_http_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<String, HttpCodegenError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;

    render_http(&index)
}
