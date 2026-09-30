use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::container_plan::ContainerPlan;
use crate::planned_provider::PlannedProvider;

pub(crate) fn construction_flow<'plan>(
    plan: &'plan ContainerPlan,
    roots: &[&'plan PlannedProvider],
) -> Vec<&'plan PlannedProvider> {
    let mut selected: BTreeSet<CanonicalPath> =
        roots.iter().map(|entry| entry.key.clone()).collect();

    for entry in plan.planned_entries().rev() {
        if !selected.contains(&entry.key) {
            continue;
        }

        for dependency in entry.provider.dependencies() {
            selected.extend(dependency.provider_keys().iter().cloned());
        }
    }

    plan.planned_entries()
        .filter(|entry| selected.contains(&entry.key))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use margaret_attributes::canonical_path::CanonicalPath;

    use super::construction_flow;
    use crate::container_plan::ContainerPlan;
    use crate::dependency_kind::DependencyKind;
    use crate::direct_construction::DirectConstruction;
    use crate::framework_injection_role::FrameworkInjectionRole;
    use crate::provided_type::ProvidedType;
    use crate::provider::Provider;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn provider(name: &str, dependencies: &[&str]) -> Provider {
        let concrete_path = path(name);

        Provider {
            concrete_path: concrete_path.clone(),
            construction: DirectConstruction::Constructor {
                dependencies: dependencies
                    .iter()
                    .map(|dependency| DependencyKind::Single {
                        provider_key: path(dependency),
                    })
                    .collect(),
                is_async: false,
                method: "create".to_string(),
            },
            field_name: name.to_lowercase(),
            injection: FrameworkInjectionRole::Unmarked,
            provided: ProvidedType::Concrete(concrete_path),
            type_name: name.to_string(),
        }
    }

    fn plan() -> ContainerPlan {
        let entries = [
            ("Base", provider("Base", &[])),
            ("Left", provider("Left", &["Base"])),
            ("Right", provider("Right", &["Base"])),
            ("Root", provider("Root", &["Left", "Right"])),
            ("Unused", provider("Unused", &[])),
        ]
        .into_iter()
        .map(|(name, provider)| (path(name), provider))
        .collect();

        ContainerPlan::new(
            ["Base", "Left", "Right", "Root", "Unused"]
                .into_iter()
                .map(path)
                .collect(),
            entries,
            BTreeSet::new(),
        )
        .expect("the fixture plan is complete")
    }

    #[test]
    fn keeps_a_shared_dependency_once_and_excludes_unused_entries() {
        let plan = plan();
        let root = plan
            .planned_entry(&path("Root"))
            .expect("the root is planned");

        assert_eq!(
            construction_flow(&plan, &[root])
                .iter()
                .map(|entry| entry.key.to_string())
                .collect::<Vec<_>>(),
            ["crate::Base", "crate::Left", "crate::Right", "crate::Root"]
        );
    }
}
