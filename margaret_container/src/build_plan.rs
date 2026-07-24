use std::collections::BTreeMap;
use std::collections::HashMap;

use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::marker::marker;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::struct_shape::StructShape;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;
use margaret_tag_codegen::read_reference_tag::read_reference_tag;
use margaret_tag_codegen::tag_kind::TagKind;
use margaret_tag_codegen::tag_pool::TagPool;

use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
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
        if matched.item().has_attribute(&provides_endpoint_selector()) {
            continue;
        }

        provider_drafts.push(build_provider_draft(&matched, index)?);
    }

    for matched in index.select(&provides_endpoint_selector()) {
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
    pool: &TagPool,
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
                pool,
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
    pool: &TagPool,
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

        if let Some(attribute) = marker(&pattern_type.attrs, &endpoint_provider_selector()) {
            dependencies.push(DependencyKind::Single {
                provider_key: resolve_endpoint_provider(
                    concrete_path,
                    &parameter,
                    attribute,
                    pool,
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

fn resolve_endpoint_provider(
    concrete_path: &CanonicalPath,
    parameter: &str,
    attribute: &Attribute,
    pool: &TagPool,
) -> Result<CanonicalPath, ContainerError> {
    let site = format!("parameter '{parameter}' of singleton '{concrete_path}'");
    let args = AttributeArgs::from_attribute(attribute)?;
    let tag = read_reference_tag(&args, &site)?;

    Ok(pool.resolve(&tag, TagKind::Endpoint, &site)?.clone())
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

fn provides_endpoint_selector() -> AttributeSelector {
    AttributeSelector::from_marker("provides_endpoint")
}

fn endpoint_provider_selector() -> AttributeSelector {
    AttributeSelector::from_marker("endpoint_provider")
}

struct Draft<'index> {
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    item: &'index IndexedItem,
    provided: ProvidedType,
}

struct DraftedContainer<'index> {
    construction_drafts: Vec<Draft<'index>>,
    provided_keys: HashMap<CanonicalPath, CanonicalPath>,
    provider_drafts: Vec<Draft<'index>>,
}

fn framework_provider_definition(framework_provider: &FrameworkProvider) -> Provider {
    match framework_provider {
        FrameworkProvider::Unit(path) => Provider {
            concrete_path: path.clone(),
            construction: DirectConstruction::Fieldless {
                shape: StructShape::Unit,
            },
            field_name: path.field_name(),
            provided: ProvidedType::Concrete(path.clone()),
        },
        FrameworkProvider::UriSelected(provider) => {
            let argument = ConsoleArgument::Named {
                name: provider.argument_name.clone(),
                required: true,
                weaving: WeavingKind::from_canonical(&provider.value_type, true),
                value_type: provider.value_type.clone(),
            };

            Provider {
                concrete_path: provider.trait_path.clone(),
                construction: DirectConstruction::Resolved {
                    dependencies: vec![DependencyKind::ConsoleArgument {
                        argument: Box::new(argument),
                    }],
                    resolver: provider.resolver.clone(),
                },
                field_name: provider.trait_path.field_name(),
                provided: ProvidedType::UriSelected(provider.trait_path.clone()),
            }
        }
    }
}

fn resolve_framework_providers(
    index: &AttributeIndex,
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    provided_keys: &mut HashMap<CanonicalPath, CanonicalPath>,
    framework_provided: &[FrameworkProvider],
) -> Result<Vec<Provider>, ContainerError> {
    let mut providers = Vec::new();

    for framework_provider in framework_provided {
        let key = framework_provider.key();

        if !is_framework_path_referenced(index, provider_drafts, key)
            && !is_framework_path_referenced(index, construction_drafts, key)
        {
            continue;
        }

        if provided_keys.insert(key.clone(), key.clone()).is_some() {
            return Err(ContainerError::AmbiguousFrameworkProvider {
                path: key.to_string(),
            });
        }

        providers.push(framework_provider_definition(framework_provider));
    }

    Ok(providers)
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

pub(crate) fn build_plan(
    index: &AttributeIndex,
    registry: &ConsoleArgumentRegistry,
    framework_provided: &[FrameworkProvider],
    pool: &TagPool,
) -> Result<ContainerPlan, ContainerError> {
    let DraftedContainer {
        construction_drafts,
        mut provided_keys,
        provider_drafts,
    } = build_drafts(index)?;

    let framework_providers = resolve_framework_providers(
        index,
        &provider_drafts,
        &construction_drafts,
        &mut provided_keys,
        framework_provided,
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
        } = draft;

        let provider_key = provided.key().clone();

        let construction = resolve_direct(
            index,
            item,
            construction,
            &concrete_path,
            &provided_keys,
            registry,
            pool,
        )?;

        providers.insert(
            provider_key,
            Provider {
                concrete_path,
                construction,
                field_name,
                provided,
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
        } = draft;

        let construction = resolve_direct(
            index,
            item,
            construction,
            &concrete_path,
            &provided_keys,
            registry,
            pool,
        )?;

        constructions.insert(
            concrete_path.clone(),
            Provider {
                concrete_path,
                construction,
                field_name,
                provided,
            },
        );
    }

    for provider in framework_providers {
        providers.insert(provider.provided.key().clone(), provider);
    }

    Ok(ContainerPlan {
        constructions,
        providers,
    })
}
