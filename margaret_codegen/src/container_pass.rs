use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::rendered_container::RenderedContainer;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::jwks_framework_providers::jwks_framework_providers;
use crate::jwks_secret_storage_provider::jwks_secret_storage_provider;
use crate::postgres_pool_provider::postgres_pool_provider;

pub(crate) fn container_pass(
    context: &mut BuildContext,
    registry: &ConsoleArgumentRegistry,
    client_bindings: &[JwksClientBinding],
) -> Result<ContainerBindings, CodegenError> {
    let mut framework_providers = vec![
        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: asset_responder_canonical_path(),
        },
        jwks_secret_storage_provider(),
    ];

    framework_providers.extend(jwks_framework_providers(client_bindings));

    let postgres_pool_enablement = if context.capabilities().has_models {
        FrameworkEnablement::Always
    } else {
        FrameworkEnablement::WhenReferenced
    };

    framework_providers.push(postgres_pool_provider(postgres_pool_enablement));

    let RenderedContainer { bindings, modules } =
        margaret_container::render_container::render_container(
            context.index(),
            registry,
            &framework_providers,
        )?;

    context.extend_modules(modules);

    Ok(bindings)
}
