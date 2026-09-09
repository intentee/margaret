use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::console_artifacts::ConsoleArtifacts;
use crate::console_plan::ConsolePlan;
use crate::render::render;

#[must_use]
pub fn render_console(plan: &ConsolePlan, bindings: &ContainerBindings) -> ConsoleArtifacts {
    let rendered = render(plan, bindings);

    ConsoleArtifacts {
        modules: vec![GeneratedModuleTokens::new("run", rendered.run)],
        construction_roots: plan.construction_roots().to_vec(),
    }
}
