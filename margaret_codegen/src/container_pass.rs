use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::rendered_container::RenderedContainer;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::jwks_secret_storage_provider::jwks_secret_storage_provider;

pub(crate) fn container_pass(
    context: &mut BuildContext,
    registry: &ConsoleArgumentRegistry,
) -> Result<ContainerBindings, CodegenError> {
    let RenderedContainer { bindings, modules } =
        margaret_container::render_container::render_container(
            context.index(),
            registry,
            &[
                FrameworkProvider::Unit(asset_responder_canonical_path()),
                jwks_secret_storage_provider(),
            ],
        )?;

    context.extend_modules(modules);

    Ok(bindings)
}
