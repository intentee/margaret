use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::unify_by_key::unify_by_key;

use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;

fn collect(
    key: &CanonicalPath,
    plan: &ContainerPlan,
    closures: &mut BTreeMap<CanonicalPath, Vec<ConsoleArgument>>,
) -> Result<Vec<ConsoleArgument>, ContainerError> {
    if let Some(existing) = closures.get(key) {
        return Ok(existing.clone());
    }

    let entry = plan
        .providers
        .get(key)
        .unwrap_or_else(|| &plan.constructions[key]);
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for dependency in entry.dependencies() {
        match dependency {
            DependencyKind::ConsoleArgument { argument } => {
                collected.push(argument.as_ref().clone());
            }
            DependencyKind::Single { provider_key } => {
                collected.extend(collect(provider_key, plan, closures)?);
            }
        }
    }

    let unified = unify_by_key(&collected)?;

    closures.insert(key.clone(), unified.clone());

    Ok(unified)
}

pub(crate) struct ConsoleClosures {
    closures: BTreeMap<CanonicalPath, Vec<ConsoleArgument>>,
    slots: BTreeMap<String, usize>,
}

impl ConsoleClosures {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Result<Self, ContainerError> {
        let mut closures: BTreeMap<CanonicalPath, Vec<ConsoleArgument>> = BTreeMap::new();

        for key in plan.providers.keys().chain(plan.constructions.keys()) {
            collect(key, plan, &mut closures)?;
        }

        let mut slots: BTreeMap<String, usize> = BTreeMap::new();
        let mut next = 0;

        for closure in closures.values() {
            for argument in closure {
                slots.entry(argument.name().to_string()).or_insert_with(|| {
                    let assigned = next;
                    next += 1;
                    assigned
                });
            }
        }

        Ok(Self { closures, slots })
    }

    pub(crate) fn of(&self, key: &CanonicalPath) -> &[ConsoleArgument] {
        &self.closures[key]
    }

    pub(crate) fn slot(&self, name: &str) -> usize {
        self.slots[name]
    }

    pub(crate) fn slots(&self) -> &BTreeMap<String, usize> {
        &self.slots
    }
}
