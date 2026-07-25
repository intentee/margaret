use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::serve_console_arguments::ServeConsoleArguments;
use margaret_service_codegen::startup_singleton::StartupSingleton;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::postgres_pool_path::postgres_pool_canonical_path;

pub(crate) fn services_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    serve_arguments: &[ConsoleArgument],
) -> Result<(), CodegenError> {
    if !context.capabilities().serves {
        return Ok(());
    }

    let postgres_pool_path = postgres_pool_canonical_path();
    let startup_singletons: Vec<StartupSingleton> = bindings
        .provider(&postgres_pool_path)
        .map(|binding| StartupSingleton {
            concrete_path: postgres_pool_path.clone(),
            field_name: binding.field_name.clone(),
        })
        .into_iter()
        .collect();

    let module = margaret_service_codegen::render_services::render_services(
        context.index(),
        context.servers(),
        context.capabilities().has_views,
        bindings,
        ServeConsoleArguments {
            serve_arguments,
            server_console_arguments: context.server_console_arguments(),
            views_console_arguments: context.views_console_arguments(),
        },
        context.framework_services(),
        &startup_singletons,
    )?;

    context.extend_modules(vec![module]);

    Ok(())
}
