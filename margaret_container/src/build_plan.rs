use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_base::field_base;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;
use margaret_tag_codegen::jwks_secret_store_target::JwksSecretStoreTarget;
use margaret_tag_codegen::read_jwks_secret_store_target::read_jwks_secret_store_target;

use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::framework_construction::FrameworkConstruction;
use crate::framework_dependency::FrameworkDependency;
use crate::framework_enablement::FrameworkEnablement;
use crate::framework_injection_role::FrameworkInjectionRole;
use crate::framework_provider::FrameworkProvider;
use crate::path_text::path_text;
use crate::peel_target::peel_target;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provides_endpoint_path::provides_endpoint_path;
use crate::resolve_construction::resolve_construction;
use crate::topological_order::topological_order;
use crate::type_text::type_text;

fn concrete_roles() -> [FrameworkAttribute; 4] {
    [
        FrameworkAttribute::Service,
        FrameworkAttribute::ScheduledWithTickTimer,
        FrameworkAttribute::HandlesMiddlewareAttribute,
        FrameworkAttribute::RendersView,
    ]
}

fn build_drafts(index: &AttributeIndex) -> Result<DraftedContainer<'_>, ContainerError> {
    let mut provider_drafts: Vec<Draft> = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::Singleton) {
        if matched
            .item()
            .has_framework_attribute(FrameworkAttribute::ProvidesJwksEndpoint)
        {
            continue;
        }

        provider_drafts.push(build_provider_draft(&matched, index)?);
    }

    for matched in index.select_framework_attribute(FrameworkAttribute::ProvidesJwksEndpoint) {
        provider_drafts.push(build_endpoint_draft(&matched, index)?);
    }

    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for draft in &provider_drafts {
        provided_keys.insert(draft.provided.key().clone(), draft.concrete_path.clone());
    }

    let mut construction_items: BTreeMap<CanonicalPath, &IndexedItem> = BTreeMap::new();

    for role in concrete_roles() {
        for matched in index.select_framework_attribute(role) {
            let item = matched.item();

            if item.has_framework_attribute(FrameworkAttribute::Singleton) {
                continue;
            }

            construction_items.insert(item.canonical_path().clone(), item);
        }
    }

    let mut construction_drafts: Vec<Draft> = Vec::new();

    for item in construction_items.into_values() {
        construction_drafts.push(build_construction_draft(item, index)?);
    }

    Ok(DraftedContainer {
        construction_drafts,
        provided_keys,
        provider_drafts,
    })
}

fn reject_singleton_arguments(
    arguments: &AttributeArgs,
    concrete_path: &CanonicalPath,
) -> Result<(), ContainerError> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(ContainerError::SingletonHasArguments {
            path: concrete_path.to_string(),
        })
    }
}

fn build_provider_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let item = matched.item();
    let (Some(identifier), ItemKind::Struct(shape)) =
        (index.struct_identifier(item.canonical_path()), item.kind())
    else {
        return Err(ContainerError::NotASingletonStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();

    reject_singleton_arguments(matched.args()?, &concrete_path)?;

    let field_name = identifier.field().to_string();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let jwks_targets = read_constructor_jwks_targets(&construction, &concrete_path)?;

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name,
        item,
        jwks_targets,
        provided: ProvidedType::Concrete(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn build_endpoint_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let item = matched.item();
    let (Some(identifier), ItemKind::Struct(shape)) =
        (index.struct_identifier(item.canonical_path()), item.kind())
    else {
        return Err(ContainerError::NotAnEndpointStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();

    let singletons = AttributeQuery::new(item).find_all_framework(FrameworkAttribute::Singleton);
    let Some(singleton) = singletons.first() else {
        return Err(ContainerError::EndpointProviderRequiresSingleton {
            path: concrete_path.to_string(),
        });
    };

    reject_singleton_arguments(singleton.args()?, &concrete_path)?;

    if has_concrete_role(item) {
        return Err(ContainerError::ConflictingEndpointRole {
            path: concrete_path.to_string(),
        });
    }

    if !implements_provides_endpoint(index, item) {
        return Err(ContainerError::EndpointProviderMissingTrait {
            path: concrete_path.to_string(),
        });
    }

    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let jwks_targets = read_constructor_jwks_targets(&construction, &concrete_path)?;

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        item,
        jwks_targets,
        provided: ProvidedType::Endpoint(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn build_construction_draft<'index>(
    item: &'index IndexedItem,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let (Some(identifier), ItemKind::Struct(shape)) =
        (index.struct_identifier(item.canonical_path()), item.kind())
    else {
        return Err(ContainerError::RoleNotAStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let jwks_targets = read_constructor_jwks_targets(&construction, &concrete_path)?;

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        item,
        jwks_targets,
        provided: ProvidedType::Concrete(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn read_constructor_jwks_targets(
    construction: &ConstructionSource<'_>,
    concrete_path: &CanonicalPath,
) -> Result<BTreeMap<usize, JwksSecretStoreTarget>, ContainerError> {
    let ConstructionSource::Constructor(constructor) = construction else {
        return Ok(BTreeMap::new());
    };
    let mut targets = BTreeMap::new();

    for parameter in constructor.parameters() {
        let Some(attribute) = parameter.framework_attribute(FrameworkAttribute::JwksSecretStore)
        else {
            continue;
        };
        let site = format!(
            "parameter '{}' of singleton '{concrete_path}'",
            parameter.diagnostic_name()
        );
        let target = read_jwks_secret_store_target(attribute.args()?, &site)?;
        targets.insert(parameter.position(), target);
    }

    Ok(targets)
}

fn has_concrete_role(item: &IndexedItem) -> bool {
    concrete_roles()
        .iter()
        .any(|role| item.has_framework_attribute(*role))
}

fn implements_provides_endpoint(index: &AttributeIndex, item: &IndexedItem) -> bool {
    item.trait_impls().iter().any(|trait_impl| {
        index
            .resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())
            .as_ref()
            == Some(&provides_endpoint_path())
    })
}

struct DependencyResolver<'resolver> {
    framework_providers: &'resolver [FrameworkProvider],
    index: &'resolver AttributeIndex,
    provided_keys: &'resolver HashMap<CanonicalPath, CanonicalPath>,
    registry: &'resolver ConsoleArgumentRegistry,
}

impl DependencyResolver<'_> {
    fn resolve_direct(
        &self,
        item: &IndexedItem,
        source: &ConstructionSource,
        jwks_targets: &BTreeMap<usize, JwksSecretStoreTarget>,
        concrete_path: &CanonicalPath,
    ) -> Result<DirectConstruction, ContainerError> {
        match source {
            ConstructionSource::Constructor(constructor) => {
                let dependencies =
                    self.resolve_dependencies(item, concrete_path, constructor, jwks_targets)?;

                Ok(DirectConstruction::Constructor {
                    dependencies,
                    is_async: constructor.signature().asyncness.is_some(),
                    method: constructor.identifier().to_string(),
                })
            }
            ConstructionSource::Fieldless(shape) => {
                Ok(DirectConstruction::Fieldless { shape: *shape })
            }
        }
    }

    fn resolve_dependencies(
        &self,
        item: &IndexedItem,
        concrete_path: &CanonicalPath,
        constructor: &IndexedMethod,
        jwks_targets: &BTreeMap<usize, JwksSecretStoreTarget>,
    ) -> Result<Vec<DependencyKind>, ContainerError> {
        let mut dependencies = Vec::new();

        if constructor.has_receiver() {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: concrete_path.to_string(),
                parameter: "self".to_string(),
                written: "self".to_string(),
            });
        }

        for indexed_parameter in constructor.parameters() {
            let parameter = indexed_parameter.diagnostic_name().to_string();
            let console_argument = self
                .registry
                .argument(concrete_path, indexed_parameter.position());
            let jwks_target = jwks_targets.get(&indexed_parameter.position());

            if let Some(attribute) =
                indexed_parameter.framework_attribute(FrameworkAttribute::SpiffeHttpClient)
            {
                if console_argument.is_some() || jwks_target.is_some() {
                    return Err(ContainerError::AmbiguousSpiffeHttpClientInjection {
                        parameter,
                        singleton: concrete_path.to_string(),
                    });
                }

                if !attribute.is_bare() {
                    return Err(ContainerError::SpiffeHttpClientTakesNoArguments {
                        parameter,
                        singleton: concrete_path.to_string(),
                    });
                }

                dependencies.push(DependencyKind::ConsoleArgument {
                    argument: Box::new(ConsoleArgument::SpiffeHttpClient),
                });

                continue;
            }

            if let Some(argument) = console_argument {
                dependencies.push(DependencyKind::ConsoleArgument {
                    argument: Box::new(argument.clone()),
                });

                continue;
            }

            if let Some(target) = jwks_target {
                dependencies.push(DependencyKind::Single {
                    provider_key: resolve_jwks_secret_store(
                        concrete_path,
                        &parameter,
                        target,
                        self.framework_providers,
                    )?,
                });

                continue;
            }

            let Some(written) = peel_target(indexed_parameter.declared()) else {
                return Err(ContainerError::UnsupportedParameterShape {
                    singleton: concrete_path.to_string(),
                    parameter,
                    written: type_text(indexed_parameter.declared()),
                });
            };

            dependencies.push(resolve_target(
                self.index,
                item,
                &written,
                concrete_path,
                &parameter,
                self.provided_keys,
            )?);
        }

        Ok(dependencies)
    }
}

fn resolve_jwks_secret_store(
    concrete_path: &CanonicalPath,
    parameter: &str,
    target: &JwksSecretStoreTarget,
    framework_providers: &[FrameworkProvider],
) -> Result<CanonicalPath, ContainerError> {
    let site = format!("parameter '{parameter}' of singleton '{concrete_path}'");

    framework_providers
        .iter()
        .find(|provider| injection_matches_target(&provider.injection, target))
        .map(|provider| provider.provided.clone())
        .ok_or_else(|| ContainerError::UnknownJwksSecretStore {
            site,
            target: jwks_store_target_description(target),
        })
}

fn injection_matches_target(role: &FrameworkInjectionRole, target: &JwksSecretStoreTarget) -> bool {
    match (role, target) {
        (FrameworkInjectionRole::JwksServerStore, JwksSecretStoreTarget::Server) => true,
        (
            FrameworkInjectionRole::JwksClientStore(role_tag),
            JwksSecretStoreTarget::Client(target_tag),
        ) => role_tag == target_tag,
        _ => false,
    }
}

fn jwks_store_target_description(target: &JwksSecretStoreTarget) -> String {
    match target {
        JwksSecretStoreTarget::Server => "the server".to_string(),
        JwksSecretStoreTarget::Client(tag) => format!("client '{tag}'"),
    }
}

fn resolve_target(
    index: &AttributeIndex,
    item: &IndexedItem,
    written: &Path,
    concrete_path: &CanonicalPath,
    parameter: &str,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
) -> Result<DependencyKind, ContainerError> {
    let provider_key = index
        .resolve_item_path(item, written)
        .filter(|path| provided_keys.contains_key(path))
        .ok_or_else(|| missing_provider(concrete_path, parameter, written))?;

    Ok(DependencyKind::Single { provider_key })
}

fn missing_provider(
    concrete_path: &CanonicalPath,
    parameter: &str,
    written: &Path,
) -> ContainerError {
    ContainerError::MissingProvider {
        singleton: concrete_path.to_string(),
        parameter: parameter.to_string(),
        written: path_text(written),
    }
}

struct Draft<'index> {
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    item: &'index IndexedItem,
    jwks_targets: BTreeMap<usize, JwksSecretStoreTarget>,
    provided: ProvidedType,
    type_name: String,
}

struct DraftedContainer<'index> {
    construction_drafts: Vec<Draft<'index>>,
    provided_keys: HashMap<CanonicalPath, CanonicalPath>,
    provider_drafts: Vec<Draft<'index>>,
}

fn resolve_framework_providers(
    index: &AttributeIndex,
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    provided_keys: &mut HashMap<CanonicalPath, CanonicalPath>,
    framework_providers: &[FrameworkProvider],
) -> Result<Vec<Provider>, ContainerError> {
    let buildable = buildable_constructions(framework_providers);
    let included = included_framework_providers(
        index,
        provider_drafts,
        construction_drafts,
        framework_providers,
        &buildable,
    );
    let mut allocator = index.reserved_allocator();
    let mut providers = Vec::new();

    let mut buildable: Vec<(CanonicalPath, DirectConstruction)> = buildable.into_iter().collect();
    buildable.sort_by(|left, right| left.0.cmp(&right.0));

    for (path, construction) in buildable {
        if !included.contains(&path) {
            continue;
        }

        if provided_keys.insert(path.clone(), path.clone()).is_some() {
            return Err(ContainerError::AmbiguousFrameworkProvider {
                path: path.to_string(),
            });
        }

        let identifier = allocator.allocate(&field_base(&path));
        let field_name = identifier.field().to_string();
        let type_name = identifier.type_name().to_string();
        let provided = framework_provided_type(&construction, path.clone());

        providers.push(Provider {
            concrete_path: path,
            construction,
            field_name,
            provided,
            type_name,
        });
    }

    Ok(providers)
}

fn framework_provided_type(construction: &DirectConstruction, path: CanonicalPath) -> ProvidedType {
    match construction {
        DirectConstruction::Resolved { .. } => ProvidedType::UriSelected(path),
        DirectConstruction::Constructor { .. }
        | DirectConstruction::Fieldless { .. }
        | DirectConstruction::FrameworkConstructor { .. }
        | DirectConstruction::FrameworkUnit
        | DirectConstruction::FrameworkAccessor { .. } => ProvidedType::Concrete(path),
    }
}

fn buildable_constructions(
    framework_providers: &[FrameworkProvider],
) -> HashMap<CanonicalPath, DirectConstruction> {
    let mut resolved: HashMap<CanonicalPath, DirectConstruction> = HashMap::new();

    for framework_provider in framework_providers {
        resolved.insert(
            framework_provider.provided.clone(),
            resolve_framework_construction(&framework_provider.construction),
        );
    }

    loop {
        let removable: Vec<CanonicalPath> = framework_providers
            .iter()
            .filter(|framework_provider| resolved.contains_key(&framework_provider.provided))
            .filter(|framework_provider| {
                framework_provider_dependencies(&framework_provider.construction)
                    .iter()
                    .any(|dependency| !resolved.contains_key(*dependency))
            })
            .map(|framework_provider| framework_provider.provided.clone())
            .collect();

        if removable.is_empty() {
            break;
        }

        for path in removable {
            resolved.remove(&path);
        }
    }

    resolved
}

fn resolve_framework_construction(construction: &FrameworkConstruction) -> DirectConstruction {
    match construction {
        FrameworkConstruction::Accessor { accessor, source } => {
            DirectConstruction::FrameworkAccessor {
                accessor: accessor.clone(),
                dependencies: vec![DependencyKind::Single {
                    provider_key: source.clone(),
                }],
            }
        }
        FrameworkConstruction::Constructor {
            dependencies,
            is_async,
            method,
        } => {
            let mut resolved = Vec::new();

            for dependency in dependencies {
                let provider_key = match dependency {
                    FrameworkDependency::Endpoint(endpoint_path) => endpoint_path.clone(),
                    FrameworkDependency::Provider(provider_key) => provider_key.clone(),
                };

                resolved.push(DependencyKind::Single { provider_key });
            }

            DirectConstruction::FrameworkConstructor {
                dependencies: resolved,
                is_async: *is_async,
                method: method.clone(),
            }
        }
        FrameworkConstruction::UriSelected {
            argument_name,
            resolver,
            value_type,
        } => DirectConstruction::Resolved {
            dependencies: vec![DependencyKind::ConsoleArgument {
                argument: Box::new(ConsoleArgument::Named {
                    name: argument_name.clone(),
                    required: true,
                    weaving: WeavingKind::from_canonical(value_type, true),
                    value_type: value_type.clone(),
                }),
            }],
            resolver: resolver.clone(),
        },
        FrameworkConstruction::Unit => DirectConstruction::FrameworkUnit,
    }
}

fn framework_provider_dependencies(construction: &FrameworkConstruction) -> Vec<&CanonicalPath> {
    match construction {
        FrameworkConstruction::Accessor { source, .. } => vec![source],
        FrameworkConstruction::Constructor { dependencies, .. } => dependencies
            .iter()
            .filter_map(|dependency| match dependency {
                FrameworkDependency::Provider(provider_key) => Some(provider_key),
                FrameworkDependency::Endpoint(_) => None,
            })
            .collect(),
        FrameworkConstruction::UriSelected { .. } | FrameworkConstruction::Unit => Vec::new(),
    }
}

fn included_framework_providers(
    index: &AttributeIndex,
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    framework_providers: &[FrameworkProvider],
    buildable: &HashMap<CanonicalPath, DirectConstruction>,
) -> HashSet<CanonicalPath> {
    let mut triggered: HashSet<CanonicalPath> = HashSet::new();

    for framework_provider in framework_providers {
        if buildable.contains_key(&framework_provider.provided)
            && is_directly_triggered(
                index,
                provider_drafts,
                construction_drafts,
                framework_provider,
            )
        {
            triggered.insert(framework_provider.provided.clone());
        }
    }

    loop {
        let mut added = false;

        for framework_provider in framework_providers {
            if !triggered.contains(&framework_provider.provided) {
                continue;
            }

            for dependency in framework_provider_dependencies(&framework_provider.construction) {
                if buildable.contains_key(dependency) && triggered.insert(dependency.clone()) {
                    added = true;
                }
            }
        }

        if !added {
            break;
        }
    }

    triggered
}

fn is_directly_triggered(
    index: &AttributeIndex,
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    framework_provider: &FrameworkProvider,
) -> bool {
    match framework_provider.enablement {
        FrameworkEnablement::Always => true,
        FrameworkEnablement::Dependency => false,
        FrameworkEnablement::WhenReferenced => {
            is_framework_path_referenced(index, provider_drafts, &framework_provider.provided)
                || is_framework_path_referenced(
                    index,
                    construction_drafts,
                    &framework_provider.provided,
                )
                || is_framework_role_referenced(provider_drafts, &framework_provider.injection)
                || is_framework_role_referenced(construction_drafts, &framework_provider.injection)
        }
    }
}

fn is_framework_path_referenced(
    index: &AttributeIndex,
    drafts: &[Draft],
    path: &CanonicalPath,
) -> bool {
    drafts
        .iter()
        .any(|draft| draft_references_path(index, draft, path))
}

fn is_framework_role_referenced(drafts: &[Draft], role: &FrameworkInjectionRole) -> bool {
    match role {
        FrameworkInjectionRole::Unmarked => false,
        FrameworkInjectionRole::JwksClientStore(_) | FrameworkInjectionRole::JwksServerStore => {
            drafts
                .iter()
                .any(|draft| draft_references_role(draft, role))
        }
    }
}

fn draft_references_path(index: &AttributeIndex, draft: &Draft, path: &CanonicalPath) -> bool {
    let ConstructionSource::Constructor(constructor) = &draft.construction else {
        return false;
    };

    constructor.parameters().iter().any(|parameter| {
        matches!(
            peel_target(parameter.declared()),
            Some(written) if index.resolve_item_path(draft.item, &written).as_ref() == Some(path)
        )
    })
}

fn draft_references_role(draft: &Draft, role: &FrameworkInjectionRole) -> bool {
    draft
        .jwks_targets
        .values()
        .any(|target| injection_matches_target(role, target))
}

pub(crate) fn build_plan(
    index: &AttributeIndex,
    registry: &ConsoleArgumentRegistry,
    framework_providers: &[FrameworkProvider],
) -> Result<ContainerPlan, ContainerError> {
    let DraftedContainer {
        construction_drafts,
        mut provided_keys,
        provider_drafts,
    } = build_drafts(index)?;

    let resolved_framework_providers = resolve_framework_providers(
        index,
        &provider_drafts,
        &construction_drafts,
        &mut provided_keys,
        framework_providers,
    )?;
    let resolver = DependencyResolver {
        framework_providers,
        index,
        provided_keys: &provided_keys,
        registry,
    };

    let mut providers = BTreeMap::new();
    let mut constructions = BTreeMap::new();

    for draft in provider_drafts {
        let Draft {
            concrete_path,
            construction,
            field_name,
            item,
            jwks_targets,
            provided,
            type_name,
        } = draft;

        let provider_key = provided.key().clone();

        let construction =
            resolver.resolve_direct(item, &construction, &jwks_targets, &concrete_path)?;

        providers.insert(
            provider_key,
            Provider {
                concrete_path,
                construction,
                field_name,
                provided,
                type_name,
            },
        );
    }

    for draft in construction_drafts {
        let Draft {
            concrete_path,
            construction,
            field_name,
            item,
            jwks_targets,
            provided,
            type_name,
        } = draft;

        let construction =
            resolver.resolve_direct(item, &construction, &jwks_targets, &concrete_path)?;

        constructions.insert(
            concrete_path.clone(),
            Provider {
                concrete_path,
                construction,
                field_name,
                provided,
                type_name,
            },
        );
    }

    for provider in resolved_framework_providers {
        providers.insert(provider.provided.key().clone(), provider);
    }

    let injectable = providers.keys().cloned().collect();
    let entries = providers.into_iter().chain(constructions).collect();
    let dependency_order = topological_order(&entries)?;

    ContainerPlan::new(dependency_order, entries, injectable)
}
