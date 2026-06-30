use margaret_attributes::canonical_path::CanonicalPath;

use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provider_construction::ProviderConstruction;
use crate::synthetic_provider::SyntheticProvider;

fn provider_of<'plan>(plan: &'plan ContainerPlan, key: &CanonicalPath) -> Option<&'plan Provider> {
    plan.providers
        .iter()
        .find(|provider| provider.provided.key() == key)
}

fn check_unique_provided(
    plan: &ContainerPlan,
    concrete_path: &CanonicalPath,
) -> Result<(), ContainerError> {
    match provider_of(plan, concrete_path) {
        Some(existing) => Err(ContainerError::SyntheticProviderConflict {
            provided: concrete_path.to_string(),
            existing: existing.concrete_path.to_string(),
        }),
        None => Ok(()),
    }
}

fn check_unique_field(
    plan: &ContainerPlan,
    field_name: &str,
    concrete_path: &CanonicalPath,
) -> Result<(), ContainerError> {
    match plan
        .providers
        .iter()
        .find(|provider| provider.field_name == field_name)
    {
        Some(existing) => Err(ContainerError::SyntheticFieldConflict {
            field: field_name.to_string(),
            first: existing.concrete_path.to_string(),
            second: concrete_path.to_string(),
        }),
        None => Ok(()),
    }
}

fn resolve_dependencies(
    plan: &ContainerPlan,
    concrete_path: &CanonicalPath,
    dependencies: &[CanonicalPath],
) -> Result<Vec<DependencyKind>, ContainerError> {
    let mut resolved = Vec::new();

    for dependency in dependencies {
        if provider_of(plan, dependency).is_none() {
            return Err(ContainerError::SyntheticDependencyMissing {
                provider: concrete_path.to_string(),
                dependency: dependency.to_string(),
            });
        }

        resolved.push(DependencyKind::Single {
            provider_key: dependency.clone(),
        });
    }

    Ok(resolved)
}

pub(crate) fn append_synthetic_providers(
    plan: &mut ContainerPlan,
    synthetic: &[SyntheticProvider],
) -> Result<(), ContainerError> {
    for provider in synthetic {
        let SyntheticProvider {
            concrete_path,
            constructor,
            dependencies,
            field_name,
        } = provider;

        check_unique_provided(plan, concrete_path)?;
        check_unique_field(plan, field_name, concrete_path)?;

        let dependencies = resolve_dependencies(plan, concrete_path, dependencies)?;

        plan.providers.push(Provider {
            concrete_path: concrete_path.clone(),
            construction: ProviderConstruction::Direct(DirectConstruction::Constructor {
                dependencies,
                is_async: false,
                method: constructor.clone(),
            }),
            field_name: field_name.clone(),
            provided: ProvidedType::Concrete(concrete_path.clone()),
        });
    }

    Ok(())
}
