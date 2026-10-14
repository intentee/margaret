use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_generated_module::generated_module::GeneratedModule;
use margaret_generated_module::module_path::ModulePath;

use crate::container_module_source::container_module_source;
use crate::generate_container_modules::generate_container_modules;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedModule, ContainerError> {
    Ok(GeneratedModule::new(
        ModulePath::root("container"),
        container_module_source(generate_container_modules(crate_name, source_directory)?),
    ))
}
