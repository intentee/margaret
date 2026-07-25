use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_base::field_base;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::marker::marker;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::struct_shape::StructShape;
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
use crate::type_text::type_text;

fn concrete_role_selectors() -> [AttributeSelector; 4] {
    [
        service_selector(),
        scheduled_with_tick_timer_selector(),
        handles_middleware_attribute_selector(),
        renders_view_selector(),
    ]
}

fn build_drafts<'index>(
    index: &'index AttributeIndex,
) -> Result<DraftedContainer<'index>, ContainerError> {
    let mut provider_drafts: Vec<Draft> = Vec::new();

    for matched in index.select(&singleton_selector()) {
        if matched.item().has_attribute(&provides_jwks_endpoint_selector()) {
            continue;
        }

        provider_drafts.push(build_provider_draft(&matched, index)?);
    }

    for matched in index.select(&provides_jwks_endpoint_selector()) {
        provider_drafts.push(build_endpoint_draft(&matched, index)?);
    }

    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for draft in &provider_drafts {
        provided_keys.insert(draft.provided.key().clone(), draft.concrete_path.clone());
    }

    let mut construction_items: BTreeMap<CanonicalPath, &IndexedItem> = BTreeMap::new();

    for selector in concrete_role_selectors() {
        for matched in index.select(&selector) {
            let item = matched.item();

            if item.has_attribute(&singleton_selector()) {
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
    let (identifier, shape) = match (index.struct_identifier(item.canonical_path()), item.kind()) {
        (Some(identifier), ItemKind::Struct(shape)) => (identifier, shape),
        _ => {
            return Err(ContainerError::NotASingletonStruct {
                path: item.canonical_path().to_string(),
            });
        }
    };

    let concrete_path = item.canonical_path().clone();

    reject_singleton_arguments(matched.args()?, &concrete_path)?;

    let field_name = identifier.field().to_string();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name,
        item,
        provided: ProvidedType::Concrete(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn build_endpoint_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let item = matched.item();
    let (identifier, shape) = match (index.struct_identifier(item.canonical_path()), item.kind()) {
        (Some(identifier), ItemKind::Struct(shape)) => (identifier, shape),
        _ => {
            return Err(ContainerError::NotAnEndpointStruct {
                path: item.canonical_path().to_string(),
            });
        }
    };

    let concrete_path = item.canonical_path().clone();

    let singletons = AttributeQuery::new(item).find_all(&singleton_selector());
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

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        item,
        provided: ProvidedType::Endpoint(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn build_construction_draft<'index>(
    item: &'index IndexedItem,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let (identifier, shape) = match (index.struct_identifier(item.canonical_path()), item.kind()) {
        (Some(identifier), ItemKind::Struct(shape)) => (identifier, shape),
        _ => {
            return Err(ContainerError::RoleNotAStruct {
                path: item.canonical_path().to_string(),
            });
        }
    };

    let concrete_path = item.canonical_path().clone();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        item,
        provided: ProvidedType::Concrete(concrete_path),
        type_name: identifier.type_name().to_string(),
    })
}

fn has_concrete_role(item: &IndexedItem) -> bool {
    concrete_role_selectors()
        .iter()
        .any(|selector| item.has_attribute(selector))
}

fn implements_provides_endpoint(index: &AttributeIndex, item: &IndexedItem) -> bool {
    item.trait_impls().iter().any(|trait_impl| {
        index
            .resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())
            .as_ref()
            == Some(&provides_endpoint_path())
    })
}

fn resolve_direct(
    index: &AttributeIndex,
    item: &IndexedItem,
    source: ConstructionSource,
    concrete_path: &CanonicalPath,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
    registry: &ConsoleArgumentRegistry,
    framework_providers: &[FrameworkProvider],
) -> Result<DirectConstruction, ContainerError> {
    match source {
        ConstructionSource::Constructor(constructor) => {
            let dependencies = resolve_dependencies(
                index,
                item,
                concrete_path,
                constructor,
                provided_keys,
                registry,
                framework_providers,
            )?;

            Ok(DirectConstruction::Constructor {
                dependencies,
                is_async: constructor.signature().asyncness.is_some(),
                method: constructor.identifier().to_string(),
            })
        }
        ConstructionSource::Fieldless(shape) => Ok(DirectConstruction::Fieldless { shape }),
    }
}

fn resolve_dependencies(
    index: &AttributeIndex,
    item: &IndexedItem,
    concrete_path: &CanonicalPath,
    constructor: &IndexedMethod,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
    registry: &ConsoleArgumentRegistry,
    framework_providers: &[FrameworkProvider],
) -> Result<Vec<DependencyKind>, ContainerError> {
    let mut dependencies = Vec::new();

    for (position, input) in constructor.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: concrete_path.to_string(),
                parameter: "self".to_string(),
                written: "self".to_string(),
            });
        };

        if let Some(argument) = registry.argument(concrete_path, position) {
            dependencies.push(DependencyKind::ConsoleArgument {
                argument: Box::new(argument.clone()),
            });

            continue;
        }

        let parameter = parameter_name(&pattern_type.pat, position);

        if let Some(attribute) = marker(&pattern_type.attrs, &jwks_secret_store_selector()) {
            dependencies.push(DependencyKind::Single {
                provider_key: resolve_jwks_secret_store(
                    concrete_path,
                    &parameter,
                    attribute,
                    framework_providers,
                )?,
            });

            continue;
        }

        let Some(written) = peel_target(&pattern_type.ty) else {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: concrete_path.to_string(),
                parameter,
                written: type_text(&pattern_type.ty),
            });
        };

        dependencies.push(resolve_target(
            index,
            item,
            &written,
            concrete_path,
            &parameter,
            provided_keys,
        )?);
    }

    Ok(dependencies)
}

fn resolve_jwks_secret_store(
    concrete_path: &CanonicalPath,
    parameter: &str,
    attribute: &Attribute,
    framework_providers: &[FrameworkProvider],
) -> Result<CanonicalPath, ContainerError> {
    let site = format!("parameter '{parameter}' of singleton '{concrete_path}'");
    let args = AttributeArgs::from_attribute(attribute)?;
    let target = read_jwks_secret_store_target(&args, &site)?;

    framework_providers
        .iter()
        .find(|provider| injection_matches_target(&provider.injection, &target))
        .map(|provider| provider.provided.clone())
        .ok_or_else(|| ContainerError::UnknownJwksSecretStore {
            site,
            target: jwks_store_target_description(&target),
        })
}

fn injection_matches_target(
    role: &FrameworkInjectionRole,
    target: &JwksSecretStoreTarget,
) -> bool {
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

fn parameter_name(pattern: &Pat, position: usize) -> String {
    match pattern {
        Pat::Ident(pattern_ident) => pattern_ident.ident.to_string(),
        _ => position.to_string(),
    }
}

pub(crate) fn singleton_selector() -> AttributeSelector {
    AttributeSelector::from_marker("singleton")
}

fn service_selector() -> AttributeSelector {
    AttributeSelector::from_marker("service")
}

fn scheduled_with_tick_timer_selector() -> AttributeSelector {
    AttributeSelector::from_marker("scheduled_with_tick_timer")
}

fn handles_middleware_attribute_selector() -> AttributeSelector {
    AttributeSelector::from_marker("handles_middleware_attribute")
}

fn renders_view_selector() -> AttributeSelector {
    AttributeSelector::from_marker("renders_view")
}

fn provides_jwks_endpoint_selector() -> AttributeSelector {
    AttributeSelector::from_marker("provides_jwks_endpoint")
}

fn jwks_secret_store_selector() -> AttributeSelector {
    AttributeSelector::from_marker("jwks_secret_store")
}

struct Draft<'index> {
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    item: &'index IndexedItem,
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

            DirectConstruction::Constructor {
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
        FrameworkConstruction::Unit => DirectConstruction::Fieldless {
            shape: StructShape::Unit,
        },
    }
}

fn framework_provider_dependencies(
    construction: &FrameworkConstruction,
) -> Vec<&CanonicalPath> {
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

    constructor.signature().inputs.iter().any(|input| {
        let FnArg::Typed(pattern_type) = input else {
            return false;
        };

        matches!(
            peel_target(&pattern_type.ty),
            Some(written) if index.resolve_item_path(draft.item, &written).as_ref() == Some(path)
        )
    })
}

fn draft_references_role(draft: &Draft, role: &FrameworkInjectionRole) -> bool {
    let ConstructionSource::Constructor(constructor) = &draft.construction else {
        return false;
    };

    constructor.signature().inputs.iter().any(|input| {
        let FnArg::Typed(pattern_type) = input else {
            return false;
        };

        marker(&pattern_type.attrs, &jwks_secret_store_selector())
            .and_then(|attribute| AttributeArgs::from_attribute(attribute).ok())
            .and_then(|args| read_jwks_secret_store_target(&args, "").ok())
            .is_some_and(|target| injection_matches_target(role, &target))
    })
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

    let mut providers = BTreeMap::new();
    let mut constructions = BTreeMap::new();

    for draft in provider_drafts {
        let Draft {
            concrete_path,
            construction,
            field_name,
            item,
            provided,
            type_name,
        } = draft;

        let provider_key = provided.key().clone();

        let construction = resolve_direct(
            index,
            item,
            construction,
            &concrete_path,
            &provided_keys,
            registry,
            framework_providers,
        )?;

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
            provided,
            type_name,
        } = draft;

        let construction = resolve_direct(
            index,
            item,
            construction,
            &concrete_path,
            &provided_keys,
            registry,
            framework_providers,
        )?;

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

    Ok(ContainerPlan {
        constructions,
        providers,
    })
}
