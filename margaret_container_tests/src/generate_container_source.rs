use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::render_container::render_container;
use margaret_generated_module::generated_module::GeneratedModule;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::container_module_source::container_module_source;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedModule, ContainerError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();
    let registry = scan(&index)?;
    let source = container_module_source(
        render_container(&index, &registry, &[], &DeclaredTokenIssuance::Absent)?.modules,
    );

    Ok(GeneratedModule::new("container", source))
}
