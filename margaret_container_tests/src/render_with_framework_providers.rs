use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

/// # Errors
///
/// Returns `ContainerError` when the fixture container cannot be rendered.
///
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn render_with_framework_providers(
    fixture: &str,
    framework_providers: &[FrameworkProvider],
) -> Result<RenderedContainer, ContainerError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture is indexed")
        .build();
    let serve_inputs = scan(&index).expect("the serve inputs are scanned");

    render_container(
        &index,
        &serve_inputs,
        framework_providers,
        &DeclaredTokenIssuance::Absent,
    )
}
