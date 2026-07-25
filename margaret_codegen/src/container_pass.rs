use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::rendered_container::RenderedContainer;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::jwks_framework_providers::jwks_framework_providers;

pub(crate) fn container_pass(
    context: &mut BuildContext,
    registry: &ConsoleArgumentRegistry,
) -> Result<ContainerBindings, CodegenError> {
    let mut framework_providers = vec![FrameworkProvider {
        construction: FrameworkConstruction::Unit,
        enablement: FrameworkEnablement::WhenReferenced,
        provided: asset_responder_canonical_path(),
    }];

    framework_providers.extend(jwks_framework_providers());

    let RenderedContainer { bindings, modules } =
        margaret_container::render_container::render_container(
            context.index(),
            registry,
            &framework_providers,
        )?;

    context.extend_modules(modules);

    Ok(bindings)
}
