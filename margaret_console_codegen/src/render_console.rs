use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::console_artifacts::ConsoleArtifacts;
use crate::console_plan::ConsolePlan;
use crate::render::render;

#[must_use]
pub fn render_console(
    plan: &ConsolePlan,
    serves: bool,
    has_models: bool,
    http_servers: &[HttpServer],
    serve_inputs: &[ServeInput],
    bindings: &ContainerBindings,
) -> ConsoleArtifacts {
    let rendered = render(
        &plan.commands,
        serves,
        has_models,
        http_servers,
        serve_inputs,
        bindings,
    );

    ConsoleArtifacts {
        modules: vec![GeneratedModuleTokens::new("run", rendered.run)],
        construction_roots: plan.construction_roots().to_vec(),
    }
}
