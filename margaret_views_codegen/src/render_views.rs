use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::render::render;
use crate::render_build::render_build;
use crate::views_artifacts::ViewsArtifacts;
use crate::views_plan::ViewsPlan;

#[must_use]
pub fn render_views(plan: ViewsPlan, bindings: &ContainerBindings) -> ViewsArtifacts {
    let views = ModulePath::root("views");

    ViewsArtifacts {
        modules: vec![
            GeneratedModuleTokens::new(views.clone(), render(&plan.views)),
            GeneratedModuleTokens::new(views.child("build"), render_build(&plan.views, bindings)),
        ],
        retained_roots: plan.retained_roots,
    }
}
