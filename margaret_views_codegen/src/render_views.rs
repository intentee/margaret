use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::render::render;
use crate::render_build::render_build;
use crate::views_artifacts::ViewsArtifacts;
use crate::views_plan::ViewsPlan;

#[must_use]
pub fn render_views(plan: ViewsPlan, bindings: &ContainerBindings) -> ViewsArtifacts {
    ViewsArtifacts {
        modules: vec![
            GeneratedModuleTokens::new("views", render(&plan.views)),
            GeneratedModuleTokens::new("views/build", render_build(&plan.views, bindings)),
        ],
        serve_inputs: plan.serve_inputs,
        retained_roots: plan.retained_roots,
    }
}
