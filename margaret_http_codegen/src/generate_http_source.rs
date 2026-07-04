use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_generated_module::generated_module::GeneratedModule;

use crate::http_codegen_error::HttpCodegenError;
use crate::render_http::render_http;

pub fn generate_http_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<String, HttpCodegenError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();

    Ok(render_http(&index)?
        .modules()
        .iter()
        .filter(|module| module.name() == "http" || module.name().starts_with("http/"))
        .map(GeneratedModule::source)
        .collect::<Vec<&str>>()
        .join("\n"))
}
