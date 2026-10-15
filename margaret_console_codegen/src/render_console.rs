use margaret_container::container_bindings::ContainerBindings;
use margaret_container::slotted_serve_input::SlottedServeInput;
use margaret_http_codegen::http_server::HttpServer;

use crate::console_artifacts::ConsoleArtifacts;
use crate::console_plan::ConsolePlan;
use crate::render::render;
use crate::serve_dispatch::ServeDispatch;

#[must_use]
pub fn render_console(
    plan: &ConsolePlan,
    serve: &ServeDispatch,
    http_servers: &[HttpServer],
    serve_inputs: &[SlottedServeInput],
    bindings: &ContainerBindings,
) -> ConsoleArtifacts {
    ConsoleArtifacts {
        modules: render(
            &plan.commands,
            &plan.framework_commands,
            serve,
            http_servers,
            serve_inputs,
            bindings,
        ),
        construction_roots: plan.construction_roots().to_vec(),
    }
}
