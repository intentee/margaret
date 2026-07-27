use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;
use margaret_console_argument_codegen::unify_by_key::unify_by_key;

use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;

pub(crate) struct ConsoleClosures {
    arguments: Vec<ConsoleArgument>,
    closures: BTreeMap<CanonicalPath, Vec<ConsoleArgument>>,
    slots: BTreeMap<ServeInputKey, usize>,
}

impl ConsoleClosures {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Result<Self, ContainerError> {
        let mut closures: BTreeMap<CanonicalPath, Vec<ConsoleArgument>> = BTreeMap::new();

        for key in &plan.dependency_order {
            let mut collected = Vec::new();

            for dependency in plan.entry(key).dependencies() {
                match dependency {
                    DependencyKind::ConsoleArgument { argument } => {
                        collected.push(argument.as_ref().clone());
                    }
                    DependencyKind::Single { provider_key } => {
                        collected.extend_from_slice(&closures[provider_key]);
                    }
                }
            }

            closures.insert(key.clone(), unify_by_key(&collected)?);
        }

        let collected = closures.values().flatten().cloned().collect::<Vec<_>>();
        let arguments = unify_by_key(&collected)?;
        let slots = arguments
            .iter()
            .enumerate()
            .map(|(slot, argument)| (argument.slot_key(), slot))
            .collect();

        Ok(Self {
            arguments,
            closures,
            slots,
        })
    }

    pub(crate) fn of(&self, key: &CanonicalPath) -> &[ConsoleArgument] {
        &self.closures[key]
    }

    pub(crate) fn slot(&self, key: &ServeInputKey) -> usize {
        self.slots[key]
    }

    pub(crate) fn slots(&self) -> &BTreeMap<ServeInputKey, usize> {
        &self.slots
    }

    pub(crate) fn argument(&self, slot: usize) -> &ConsoleArgument {
        &self.arguments[slot]
    }
}
