use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::console_closures::ConsoleClosures;
use crate::container_bindings::ContainerBindings;
use crate::container_plan::ContainerPlan;
use crate::render::render;
use crate::render_build::render_build;
use crate::rendered_container::RenderedContainer;

pub struct PlannedContainer {
    bindings: ContainerBindings,
    closures: ConsoleClosures,
    plan: ContainerPlan,
}

impl PlannedContainer {
    pub(crate) fn new(
        bindings: ContainerBindings,
        closures: ConsoleClosures,
        plan: ContainerPlan,
    ) -> Self {
        Self {
            bindings,
            closures,
            plan,
        }
    }

    #[must_use]
    pub fn bindings(&self) -> &ContainerBindings {
        &self.bindings
    }

    #[must_use]
    pub fn render(self, serve_roots: &[CanonicalPath]) -> RenderedContainer {
        let modules = vec![
            GeneratedModuleTokens::new("container", render(&self.plan, serve_roots)),
            GeneratedModuleTokens::new(
                "container/build",
                render_build(&self.plan, &self.closures, serve_roots),
            ),
        ];

        RenderedContainer {
            bindings: self.bindings,
            modules,
        }
    }

    #[must_use]
    pub fn roots(&self) -> Vec<CanonicalPath> {
        self.plan.dependency_order.clone()
    }
}
