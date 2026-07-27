use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::render::render;
use crate::render_build::render_build;
use crate::rendered_container::RenderedContainer;

pub struct PlannedContainer {
    bindings: ContainerBindings,
    plan: ContainerPlan,
}

impl PlannedContainer {
    pub(crate) fn new(bindings: ContainerBindings, plan: ContainerPlan) -> Self {
        Self { bindings, plan }
    }

    #[must_use]
    pub fn bindings(&self) -> &ContainerBindings {
        &self.bindings
    }

    pub fn render(
        self,
        construction_roots: &[CanonicalPath],
        retained_roots: &[CanonicalPath],
        builder_roots: &[CanonicalPath],
    ) -> Result<RenderedContainer, ContainerError> {
        let construction_roots = construction_roots
            .iter()
            .map(|root| self.plan.planned_entry(root))
            .collect::<Result<Vec<_>, _>>()?;
        let retained_roots = retained_roots
            .iter()
            .map(|root| self.plan.planned_entry(root))
            .collect::<Result<Vec<_>, _>>()?;
        let builder_roots = builder_roots
            .iter()
            .map(|root| self.plan.planned_entry(root))
            .collect::<Result<Vec<_>, _>>()?;
        let modules = vec![
            GeneratedModuleTokens::new("container", render(&construction_roots, &retained_roots)),
            GeneratedModuleTokens::new(
                "container/build",
                render_build(
                    &self.plan,
                    &construction_roots,
                    &retained_roots,
                    &builder_roots,
                ),
            ),
        ];

        Ok(RenderedContainer {
            bindings: self.bindings,
            modules,
        })
    }

    #[must_use]
    pub fn roots(&self) -> Vec<CanonicalPath> {
        self.plan.roots().cloned().collect()
    }
}
