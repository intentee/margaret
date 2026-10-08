use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::construction_flow::construction_flow;
use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::provider_requirement::ProviderRequirement;
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

    /// # Errors
    ///
    /// Returns `ContainerError::UnconsumedSingleton` for a singleton and
    /// `ContainerError::UnconsumedDeclaration` for a declared framework provider that no root
    /// reaches.
    pub fn render(
        self,
        served_roots: &[CanonicalPath],
        builder_roots: &[CanonicalPath],
    ) -> Result<RenderedContainer, ContainerError> {
        let served_roots = served_roots
            .iter()
            .map(|root| self.plan.planned_entry(root))
            .collect::<Result<Vec<_>, _>>()?;
        let builder_roots = builder_roots
            .iter()
            .map(|root| self.plan.planned_entry(root))
            .collect::<Result<Vec<_>, _>>()?;
        let consumed: BTreeSet<&CanonicalPath> = construction_flow(&self.plan, &served_roots)
            .into_iter()
            .chain(
                builder_roots
                    .iter()
                    .flat_map(|root| construction_flow(&self.plan, &[*root])),
            )
            .map(|entry| &entry.key)
            .collect();

        for entry in self.plan.planned_entries() {
            if consumed.contains(&entry.key) {
                continue;
            }

            match entry.provider.requirement {
                ProviderRequirement::Declared => {
                    return Err(ContainerError::UnconsumedDeclaration {
                        path: entry.key.to_string(),
                    });
                }
                ProviderRequirement::Singleton => {
                    return Err(ContainerError::UnconsumedSingleton {
                        path: entry.key.to_string(),
                    });
                }
                ProviderRequirement::Optional => {}
            }
        }

        let mut modules = vec![GeneratedModuleTokens::new(
            "container",
            render(&served_roots),
        )];
        modules.extend(render_build(&self.plan, &served_roots, &builder_roots));

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
