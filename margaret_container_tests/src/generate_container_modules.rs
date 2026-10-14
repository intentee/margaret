use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::render_container::render_container;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn generate_container_modules(
    crate_name: &str,
    source_directory: &Path,
) -> Result<Vec<GeneratedModuleTokens>, ContainerError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();
    let registry = scan(&index)?;

    Ok(render_container(
        &index,
        &registry,
        &[],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::Absent,
    )?
    .modules)
}
