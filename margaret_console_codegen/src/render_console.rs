use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;

use crate::console_artifacts::ConsoleArtifacts;
use crate::console_plan::ConsolePlan;
use crate::render::render;

#[must_use]
pub fn render_console(
    plan: &ConsolePlan,
    serves: bool,
    has_models: bool,
    http_servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> ConsoleArtifacts {
    ConsoleArtifacts {
        module: GeneratedModuleTokens::new(
            "run",
            render(
                &plan.commands,
                serves,
                has_models,
                http_servers,
                serve_arguments,
                bindings,
            ),
        ),
        construction_roots: plan.construction_roots().to_vec(),
    }
}
