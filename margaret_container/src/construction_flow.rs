use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;

pub(crate) fn construction_flow(
    plan: &ContainerPlan,
    roots: &[CanonicalPath],
) -> Vec<CanonicalPath> {
    let mut selected: BTreeSet<CanonicalPath> = roots.iter().cloned().collect();

    for key in plan.dependency_order.iter().rev() {
        if !selected.contains(key) {
            continue;
        }

        for dependency in plan.entry(key).dependencies() {
            if let DependencyKind::Single { provider_key } = dependency {
                selected.insert(provider_key.clone());
            }
        }
    }

    plan.dependency_order
        .iter()
        .filter(|key| selected.contains(*key))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::construction_flow;
    use crate::container_plan::ContainerPlan;
    use crate::dependency_kind::DependencyKind;
    use crate::direct_construction::DirectConstruction;
    use crate::provided_type::ProvidedType;
    use crate::provider::Provider;
    use margaret_attributes::canonical_path::CanonicalPath;

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

        ContainerPlan {
            dependency_order: ["Base", "Left", "Right", "Root", "Unused"]
                .into_iter()
                .map(path)
                .collect(),
            entries,
            injectable: BTreeSet::new(),
        }
    }

    #[test]
    fn keeps_a_shared_dependency_once_and_excludes_unused_entries() {
        assert_eq!(
            construction_flow(&plan(), &[path("Root")])
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["crate::Base", "crate::Left", "crate::Right", "crate::Root"]
        );
    }
}
