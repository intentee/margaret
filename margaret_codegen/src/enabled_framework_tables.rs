use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_provider::FrameworkProvider;

pub(crate) fn enabled_framework_tables(
    providers: &[FrameworkProvider],
    bindings: &ContainerBindings,
) -> Vec<CanonicalPath> {
    let mut enabled: Vec<CanonicalPath> = Vec::new();

    for provider in providers
        .iter()
        .filter(|provider| bindings.provides(&provider.provided))
    {
        if let FrameworkConstruction::Constructor { dependencies, .. } = &provider.construction {
            for dependency in dependencies {
                if let FrameworkDependency::Database { framework_tables } = dependency {
                    for tables in framework_tables {
                        if !enabled.contains(tables) {
                            enabled.push(tables.clone());
                        }
                    }
                }
            }
        }
    }

    enabled
}
