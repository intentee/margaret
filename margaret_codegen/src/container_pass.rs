use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::framework_provider_construction::FrameworkProviderConstruction;
use margaret_container::rendered_container::RenderedContainer;
use margaret_service_codegen::identity_framework_providers::identity_framework_providers;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

fn framework_provided() -> Vec<FrameworkProvider> {
    let mut providers = vec![FrameworkProvider {
        construction: FrameworkProviderConstruction::Fieldless,
        path: asset_responder_canonical_path(),
    }];

    providers.extend(identity_framework_providers());

    providers
}

pub(crate) fn container_pass(
    context: &mut BuildContext,
    registry: &ConsoleArgumentRegistry,
) -> Result<ContainerBindings, CodegenError> {
    let RenderedContainer { bindings, modules } =
        margaret_container::render_container::render_container(
            context.index(),
            registry,
            &framework_provided(),
        )?;

    context.extend_modules(modules);

    Ok(bindings)
}
