use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

use crate::http_codegen_error::HttpCodegenError;
use crate::render_http::render_http;

pub fn generate_http_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<String, HttpCodegenError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();

    Ok(render_http(&index)?.source().to_string())
}
