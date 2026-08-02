use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;
use margaret_console_argument_codegen::serve_input_registry::ServeInputRegistry;

use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::planned_dependency::PlannedDependency;
use crate::planned_provider::PlannedProvider;
use crate::provider::Provider;

pub(crate) struct ContainerPlan {
    arguments: Arc<[ConsoleArgument]>,
    entries: Vec<PlannedProvider>,
    positions: BTreeMap<CanonicalPath, usize>,
    injectable: BTreeSet<CanonicalPath>,
    slots: Arc<BTreeMap<ServeInputKey, usize>>,
}

impl ContainerPlan {
    pub(crate) fn new(
        dependency_order: Vec<CanonicalPath>,
        mut entries: BTreeMap<CanonicalPath, Provider>,
        injectable: BTreeSet<CanonicalPath>,
    ) -> Result<Self, ContainerError> {
        let mut ordered: Vec<PlannedProvider> = Vec::with_capacity(dependency_order.len());
        let mut positions: BTreeMap<CanonicalPath, usize> = BTreeMap::new();
        let mut argument_registry = ServeInputRegistry::empty();

        for key in dependency_order {
            let provider =
                entries
                    .remove(&key)
                    .ok_or_else(|| ContainerError::MissingPlannedProvider {
                        path: key.to_string(),
                    })?;
            let mut collected = Vec::new();
            let mut collected_slots = Vec::new();
            let mut seen_slots = BTreeSet::new();
            let mut dependencies = Vec::new();
            let mut is_async = provider.construction.is_async();

            for dependency in provider.dependencies() {
                match dependency {
                    DependencyKind::ConsoleArgument { argument } => {
                        let slot = argument_registry.register(argument.as_ref().clone())?;
                        dependencies.push(PlannedDependency::ConsoleArgument {
                            argument: argument.as_ref().clone(),
                            slot,
                        });

                        if seen_slots.insert(slot) {
                            collected.push(argument.as_ref().clone());
                            collected_slots.push(slot);
                        }
                    }
                    DependencyKind::Single { provider_key } => {
                        let dependency = positions
                            .get(provider_key)
                            .and_then(|position| ordered.get(*position))
                            .ok_or_else(|| ContainerError::MissingPlannedProvider {
                                path: provider_key.to_string(),
                            })?;
                        dependencies.push(PlannedDependency::Single {
                            field_name: dependency.provider.field_name.clone(),
                        });

                        for (argument, slot) in dependency
                            .console_arguments
                            .iter()
                            .zip(dependency.console_slots.iter().copied())
                        {
                            if seen_slots.insert(slot) {
                                collected.push(argument.clone());
                                collected_slots.push(slot);
                            }
                        }
                        is_async |= dependency.is_async;
                    }
                }
            }

            positions.insert(key.clone(), ordered.len());
            ordered.push(PlannedProvider {
                console_arguments: collected.into(),
                console_slots: collected_slots.into(),
                dependencies: dependencies.into(),
                is_async,
                key,
                provider,
            });
        }

        let ServeInputRegistry { arguments, slots } = argument_registry;

        Ok(Self {
            arguments: arguments.into(),
            entries: ordered,
            positions,
            injectable,
            slots: Arc::new(slots),
        })
    }

    pub(crate) fn arguments(&self) -> Arc<[ConsoleArgument]> {
        Arc::clone(&self.arguments)
    }

    pub(crate) fn injectable(&self, key: &CanonicalPath) -> bool {
        self.injectable.contains(key)
    }

    pub(crate) fn planned_entries(&self) -> impl DoubleEndedIterator<Item = &PlannedProvider> {
        self.entries.iter()
    }

    pub(crate) fn planned_entry(
        &self,
        key: &CanonicalPath,
    ) -> Result<&PlannedProvider, ContainerError> {
        self.positions
            .get(key)
            .and_then(|position| self.entries.get(*position))
            .ok_or_else(|| ContainerError::MissingPlannedProvider {
                path: key.to_string(),
            })
    }

    pub(crate) fn roots(&self) -> impl Iterator<Item = &CanonicalPath> {
        self.entries.iter().map(|entry| &entry.key)
    }

    pub(crate) fn slots(&self) -> Arc<BTreeMap<ServeInputKey, usize>> {
        Arc::clone(&self.slots)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_console_argument_codegen::weaving_kind::WeavingKind;

    use super::ContainerPlan;
    use crate::dependency_kind::DependencyKind;
    use crate::direct_construction::DirectConstruction;
    use crate::provided_type::ProvidedType;
    use crate::provider::Provider;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn provider_with_missing_dependency() -> Provider {
        let concrete_path = path("Root");

        Provider {
            concrete_path: concrete_path.clone(),
            construction: DirectConstruction::Constructor {
                dependencies: vec![DependencyKind::Single {
                    provider_key: path("Missing"),
                }],
                is_async: false,
                method: "create".to_string(),
            },
            field_name: "root".to_string(),
            provided: ProvidedType::Concrete(concrete_path),
            type_name: "Root".to_string(),
        }
    }

    fn repeated_console_argument_provider() -> Provider {
        let concrete_path = path("Root");
        let argument = || DependencyKind::ConsoleArgument {
            argument: Box::new(ConsoleArgument::Named {
                name: "shared".to_string(),
                required: true,
                weaving: WeavingKind::Cloned,
                value_type: path("String"),
            }),
        };

        Provider {
            concrete_path: concrete_path.clone(),
            construction: DirectConstruction::Constructor {
                dependencies: vec![argument(), argument()],
                is_async: false,
                method: "create".to_string(),
            },
            field_name: "root".to_string(),
            provided: ProvidedType::Concrete(concrete_path),
            type_name: "Root".to_string(),
        }
    }

    #[test]
    fn rejects_an_order_entry_without_a_provider() {
        let error = ContainerPlan::new(vec![path("Missing")], BTreeMap::new(), BTreeSet::new())
            .err()
            .expect("every ordered provider must be present");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn reports_a_provider_absent_from_a_complete_plan() {
        let plan = ContainerPlan::new(Vec::new(), BTreeMap::new(), BTreeSet::new())
            .expect("an empty plan is complete");
        let error = plan
            .planned_entry(&path("Missing"))
            .err()
            .expect("the provider is absent");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn rejects_a_dependency_that_precedes_no_planned_provider() {
        let entries = BTreeMap::from([(path("Root"), provider_with_missing_dependency())]);
        let error = ContainerPlan::new(vec![path("Root")], entries, BTreeSet::new())
            .err()
            .expect("the dependency must already be structurally planned");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn assigns_one_input_slot_to_repeated_constructor_arguments() {
        let root = path("Root");
        let plan = ContainerPlan::new(
            vec![root.clone()],
            BTreeMap::from([(root.clone(), repeated_console_argument_provider())]),
            BTreeSet::new(),
        )
        .expect("repeated named inputs share one slot");
        let planned = plan
            .planned_entry(&root)
            .expect("the root belongs to the plan");

        assert_eq!(planned.console_arguments.len(), 1);
        assert_eq!(planned.console_slots.len(), 1);
        assert_eq!(planned.dependencies.len(), 2);
    }
}
