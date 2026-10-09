use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::serve_input_naming::ServeInputNaming;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_registry::ServeInputRegistry;

use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::planned_dependency::PlannedDependency;
use crate::planned_field::PlannedField;
use crate::planned_provider::PlannedProvider;
use crate::planned_url::PlannedUrl;
use crate::provider::Provider;
use crate::slotted_serve_input::SlottedServeInput;
use crate::url_source::UrlSource;

struct DependencyCollector {
    collected: Vec<SlottedServeInput>,
    is_async: bool,
    seen_slots: BTreeSet<usize>,
}

impl DependencyCollector {
    fn collect(&mut self, input: &ServeInput, slot: usize) {
        if self.seen_slots.insert(slot) {
            self.collected.push(SlottedServeInput {
                input: input.clone(),
                slot,
            });
        }
    }

    fn field(
        &mut self,
        positions: &BTreeMap<CanonicalPath, usize>,
        ordered: &[PlannedProvider],
        provider_key: &CanonicalPath,
    ) -> Result<PlannedField, ContainerError> {
        let planned = positions
            .get(provider_key)
            .and_then(|position| ordered.get(*position))
            .ok_or_else(|| ContainerError::MissingPlannedProvider {
                path: provider_key.to_string(),
            })?;

        for slotted in planned.serve_inputs.iter() {
            self.collect(&slotted.input, slotted.slot);
        }

        self.is_async |= planned.is_async;

        Ok(PlannedField {
            concrete_path: planned.provider.concrete_path.clone(),
            field_name: planned.provider.field_name.clone(),
        })
    }

    fn fields(
        &mut self,
        positions: &BTreeMap<CanonicalPath, usize>,
        ordered: &[PlannedProvider],
        provider_keys: &[CanonicalPath],
    ) -> Result<Vec<PlannedField>, ContainerError> {
        provider_keys
            .iter()
            .map(|provider_key| self.field(positions, ordered, provider_key))
            .collect()
    }

    fn plan(
        &mut self,
        dependency: &DependencyKind,
        key: &CanonicalPath,
        positions: &BTreeMap<CanonicalPath, usize>,
        ordered: &[PlannedProvider],
        input_registry: &mut ServeInputRegistry,
    ) -> Result<PlannedDependency, ContainerError> {
        match dependency {
            DependencyKind::Collection { provider_keys } => self
                .fields(positions, ordered, provider_keys)
                .map(PlannedDependency::Collection),
            DependencyKind::Constant { path } => Ok(PlannedDependency::Constant(path.clone())),
            DependencyKind::ServeInput { input } => input_registry
                .register(key, input.as_ref().clone())
                .map_err(ContainerError::from)
                .map(|slot| {
                    self.collect(input, slot);

                    PlannedDependency::ServeInput {
                        input: input.as_ref().clone(),
                        slot,
                    }
                }),
            DependencyKind::Single { provider_key } => self
                .field(positions, ordered, provider_key)
                .map(PlannedDependency::Single),
            DependencyKind::Urls { sources } => sources
                .iter()
                .map(|source| match source {
                    UrlSource::Declared(url) => Ok(PlannedUrl::Declared(url.clone())),
                    UrlSource::Route(route) => {
                        let input = ServeInput::RouteUrl(route.clone());

                        input_registry
                            .register(key, input.clone())
                            .map_err(ContainerError::from)
                            .map(|slot| {
                                self.collect(&input, slot);

                                PlannedUrl::Route { input, slot }
                            })
                    }
                })
                .collect::<Result<Vec<PlannedUrl>, ContainerError>>()
                .map(PlannedDependency::Urls),
        }
    }
}

pub(crate) struct ContainerPlan {
    entries: Vec<PlannedProvider>,
    positions: BTreeMap<CanonicalPath, usize>,
    injectable: BTreeSet<CanonicalPath>,
    serve_input_naming: ServeInputNaming,
}

impl ContainerPlan {
    pub(crate) fn new(
        dependency_order: Vec<CanonicalPath>,
        mut entries: BTreeMap<CanonicalPath, Provider>,
        injectable: BTreeSet<CanonicalPath>,
    ) -> Result<Self, ContainerError> {
        let mut ordered: Vec<PlannedProvider> = Vec::with_capacity(dependency_order.len());
        let mut positions: BTreeMap<CanonicalPath, usize> = BTreeMap::new();
        let mut input_registry = ServeInputRegistry::empty();

        for key in dependency_order {
            let provider =
                entries
                    .remove(&key)
                    .ok_or_else(|| ContainerError::MissingPlannedProvider {
                        path: key.to_string(),
                    })?;
            let mut collector = DependencyCollector {
                collected: Vec::new(),
                is_async: provider.construction.is_async(),
                seen_slots: BTreeSet::new(),
            };
            let dependencies = provider
                .dependencies()
                .iter()
                .map(|dependency| {
                    collector.plan(dependency, &key, &positions, &ordered, &mut input_registry)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let DependencyCollector {
                collected,
                is_async,
                ..
            } = collector;

            positions.insert(key.clone(), ordered.len());
            ordered.push(PlannedProvider {
                dependencies: dependencies.into(),
                is_async,
                key,
                provider,
                serve_inputs: collected.into(),
            });
        }

        Ok(Self {
            entries: ordered,
            serve_input_naming: ServeInputNaming::for_slot_count(input_registry.slot_count()),
            positions,
            injectable,
        })
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

    pub(crate) fn serve_input_naming(&self) -> ServeInputNaming {
        self.serve_input_naming
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;
    use margaret_serve_input_codegen::serve_input::ServeInput;

    use super::ContainerPlan;
    use crate::dependency_kind::DependencyKind;
    use crate::direct_construction::DirectConstruction;
    use crate::framework_injection_role::FrameworkInjectionRole;
    use crate::provider::Provider;
    use crate::provider_requirement::ProviderRequirement;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn provider_depending_on(dependency: DependencyKind) -> Provider {
        let concrete_path = path("Root");

        Provider {
            concrete_path: concrete_path.clone(),
            construction: DirectConstruction::Constructor {
                dependencies: vec![dependency],
                is_async: false,
                method: "create".to_string(),
            },
            field_name: "root".to_string(),
            injection: FrameworkInjectionRole::Unmarked,
            requirement: ProviderRequirement::Singleton,
            type_name: "Root".to_string(),
        }
    }

    fn repeated_serve_input_provider() -> Provider {
        let concrete_path = path("Root");
        let input = || DependencyKind::ServeInput {
            input: Box::new(ServeInput::ConsoleArgument(ConsoleArgument::Named {
                name: "shared".to_string(),
                value: InputValue {
                    required: true,
                    value_type: path("String"),
                    weaving: WeavingKind::Cloned,
                },
            })),
        };

        Provider {
            concrete_path: concrete_path.clone(),
            construction: DirectConstruction::Constructor {
                dependencies: vec![input(), input()],
                is_async: false,
                method: "create".to_string(),
            },
            field_name: "root".to_string(),
            injection: FrameworkInjectionRole::Unmarked,
            requirement: ProviderRequirement::Singleton,
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
        let entries = BTreeMap::from([(
            path("Root"),
            provider_depending_on(DependencyKind::Single {
                provider_key: path("Missing"),
            }),
        )]);
        let error = ContainerPlan::new(vec![path("Root")], entries, BTreeSet::new())
            .err()
            .expect("the dependency must already be structurally planned");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn rejects_a_collected_provider_that_precedes_no_planned_provider() {
        let entries = BTreeMap::from([(
            path("Root"),
            provider_depending_on(DependencyKind::Collection {
                provider_keys: vec![path("Missing")],
            }),
        )]);
        let error = ContainerPlan::new(vec![path("Root")], entries, BTreeSet::new())
            .err()
            .expect("the collected provider must already be structurally planned");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn assigns_one_input_slot_to_repeated_constructor_arguments() {
        let root = path("Root");
        let plan = ContainerPlan::new(
            vec![root.clone()],
            BTreeMap::from([(root.clone(), repeated_serve_input_provider())]),
            BTreeSet::new(),
        )
        .expect("repeated named inputs share one slot");
        let planned = plan
            .planned_entry(&root)
            .expect("the root belongs to the plan");

        assert_eq!(planned.serve_inputs.len(), 1);
        assert_eq!(planned.serve_inputs.len(), 1);
        assert_eq!(planned.dependencies.len(), 2);
    }
}
