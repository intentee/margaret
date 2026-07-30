use std::path::PathBuf;

use margaret_attributes::crate_root::CrateRoot;
use margaret_codegen::build::build;
use margaret_codegen::codegen_error::CodegenError;
use margaret_codegen::generated_code::GeneratedCode;

/// # Errors
///
/// Returns `CodegenError` propagated from the work it performs.
pub fn generate_fixture(name: &str) -> Result<GeneratedCode, CodegenError> {
    let source_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
        .join("src");
    let assets_directory = source_directory.join("assets");

    build(
        &CrateRoot::new(name, source_directory),
        None,
        &assets_directory,
        ".",
    )
}
