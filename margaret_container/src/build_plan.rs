use std::collections::BTreeMap;
use std::collections::HashMap;

use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::marker::marker;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::struct_shape::StructShape;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_tag_codegen::read_reference_tag::read_reference_tag;
use margaret_tag_codegen::tag_kind::TagKind;
use margaret_tag_codegen::tag_pool::TagPool;

use crate::collection_table::CollectionTable;
use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::managed_arguments::ManagedArguments;
use crate::path_text::path_text;
use crate::peel_target::peel_target;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provides_endpoint_path::provides_endpoint_path;
use crate::raw_target::RawTarget;
use crate::resolve_construction::resolve_construction;
use crate::type_text::type_text;

fn managed_selectors() -> [AttributeSelector; 3] {
    [
        singleton_selector(),
        service_selector(),
        scheduled_with_tick_timer_selector(),
    ]
}

fn build_drafts<'index>(
    index: &'index AttributeIndex,
) -> Result<DraftedProviders<'index>, ContainerError> {
    let mut drafts: Vec<SingletonDraft> = Vec::new();

    for selector in managed_selectors() {
        for matched in index.select(&selector) {
            drafts.push(build_draft(&matched, index)?);
        }
    }

    for matched in index.select(&provides_endpoint_selector()) {
        drafts.push(build_endpoint_draft(&matched, index)?);
    }

    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for draft in &drafts {
        if let Some(first) =
            provided_keys.insert(draft.provided.key().clone(), draft.concrete_path.clone())
        {
            return Err(ContainerError::DuplicateProvider {
                provided: draft.provided.key().to_string(),
                first: first.to_string(),
                second: draft.concrete_path.to_string(),
            });
        }
    }

    Ok(DraftedProviders {
        drafts,
        provided_keys,
    })
}

fn build_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
) -> Result<SingletonDraft<'index>, ContainerError> {
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
    let field_name = identifier.field().to_string();
    let ManagedArguments {
        collection,
        provides,
    } = ManagedArguments::parse(matched.args()?)?;
    let provided = resolve_provided(index, item, provides.as_ref(), &concrete_path)?;

    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let collection = resolve_collection(index, item, collection.as_ref(), &concrete_path)?;

    Ok(SingletonDraft {
        collection,
        concrete_path,
        construction,
        field_name,
        item,
        provided,
    })
}

fn build_endpoint_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
) -> Result<SingletonDraft<'index>, ContainerError> {
    let item = matched.item();
    let (identifier, shape) = match (index.struct_identifier(item.canonical_path()), item.kind()) {
        (Some(identifier), ItemKind::Struct(shape)) => (identifier, shape),
        _ => {
            return Err(ContainerError::NotAnEndpointStruct {
                path: item.canonical_path().to_string(),
            });
        }
    };

    if conflicts_with_managed_role(item) {
        return Err(ContainerError::ConflictingEndpointRole {
            path: item.canonical_path().to_string(),
        });
    }

    if !implements_provides_endpoint(index, item) {
        return Err(ContainerError::EndpointProviderMissingTrait {
            path: item.canonical_path().to_string(),
        });
    }

    let concrete_path = item.canonical_path().clone();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;

    Ok(SingletonDraft {
        collection: None,
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        item,
        provided: ProvidedType::Endpoint(concrete_path),
    })
}

fn conflicts_with_managed_role(item: &IndexedItem) -> bool {
    managed_selectors()
        .iter()
        .any(|selector| has_marker(item, selector))
}

fn has_marker(item: &IndexedItem, selector: &AttributeSelector) -> bool {
    item.attributes()
        .iter()
        .any(|attribute| selector.matches(attribute.path()))
}

fn implements_provides_endpoint(index: &AttributeIndex, item: &IndexedItem) -> bool {
    item.trait_impls().iter().any(|trait_impl| {
        index
            .resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())
            .as_ref()
            == Some(&provides_endpoint_path())
    })
}

fn resolve_provided(
    index: &AttributeIndex,
    item: &IndexedItem,
    provides: Option<&Path>,
    concrete_path: &CanonicalPath,
) -> Result<ProvidedType, ContainerError> {
    match provides {
        Some(written) => resolve_interface(index, item, written, concrete_path),
        None => Ok(ProvidedType::Concrete(concrete_path.clone())),
    }
}

fn resolve_interface(
    index: &AttributeIndex,
    item: &IndexedItem,
    written: &Path,
    concrete_path: &CanonicalPath,
) -> Result<ProvidedType, ContainerError> {
    match index.resolve_item_path(item, written) {
        Some(path) if index.is_indexed_trait(&path) => Ok(ProvidedType::Interface(path)),
        _ => Err(ContainerError::ProvidesUnresolvable {
            singleton: concrete_path.to_string(),
            written: path_text(written),
        }),
    }
}

fn resolve_collection(
    index: &AttributeIndex,
    item: &IndexedItem,
    collection: Option<&Path>,
    concrete_path: &CanonicalPath,
) -> Result<Option<CanonicalPath>, ContainerError> {
    match collection {
        Some(written) => match index.resolve_item_path(item, written) {
            Some(path) if index.is_indexed_trait(&path) => Ok(Some(path)),
            _ => Err(ContainerError::CollectionUnresolvable {
                singleton: concrete_path.to_string(),
                written: path_text(written),
            }),
        },
        None => Ok(None),
    }
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

        let Some(target) = peel_target(&pattern_type.ty) else {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: concrete_path.to_string(),
                parameter,
                written: type_text(&pattern_type.ty),
            });
        };

        dependencies.push(resolve_target(
            index,
            item,
            target,
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
    target: RawTarget,
    concrete_path: &CanonicalPath,
    parameter: &str,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
) -> Result<DependencyKind, ContainerError> {
    match target {
        RawTarget::Single(written) => {
            let provider_key = index
                .resolve_item_path(item, &written)
                .filter(|path| provided_keys.contains_key(path))
                .ok_or_else(|| missing_provider(concrete_path, parameter, &written))?;

            Ok(DependencyKind::Single { provider_key })
        }
        RawTarget::Collection(written) => {
            let trait_path = index
                .resolve_item_path(item, &written)
                .filter(|path| index.is_indexed_trait(path))
                .ok_or_else(|| missing_provider(concrete_path, parameter, &written))?;

            Ok(DependencyKind::Collection { trait_path })
        }
    }
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

fn singleton_selector() -> AttributeSelector {
    AttributeSelector::from_marker("singleton")
}

fn service_selector() -> AttributeSelector {
    AttributeSelector::from_marker("service")
}

fn scheduled_with_tick_timer_selector() -> AttributeSelector {
    AttributeSelector::from_marker("scheduled_with_tick_timer")
}

fn provides_endpoint_selector() -> AttributeSelector {
    AttributeSelector::from_marker("provides_endpoint")
}

fn endpoint_provider_selector() -> AttributeSelector {
    AttributeSelector::from_marker("endpoint_provider")
}

struct SingletonDraft<'index> {
    collection: Option<CanonicalPath>,
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    item: &'index IndexedItem,
    provided: ProvidedType,
}

struct DraftedProviders<'index> {
    drafts: Vec<SingletonDraft<'index>>,
    provided_keys: HashMap<CanonicalPath, CanonicalPath>,
}

fn resolve_framework_providers(
    index: &AttributeIndex,
    drafts: &[SingletonDraft],
    provided_keys: &mut HashMap<CanonicalPath, CanonicalPath>,
    framework_provided: &[CanonicalPath],
) -> Result<Vec<Provider>, ContainerError> {
    let mut providers = Vec::new();

    for path in framework_provided {
        if !is_framework_path_referenced(index, drafts, path) {
            continue;
        }

        if provided_keys.insert(path.clone(), path.clone()).is_some() {
            return Err(ContainerError::AmbiguousFrameworkProvider {
                path: path.to_string(),
            });
        }

        providers.push(Provider {
            concrete_path: path.clone(),
            construction: DirectConstruction::Fieldless {
                shape: StructShape::Unit,
            },
            field_name: path.field_name(),
            provided: ProvidedType::Concrete(path.clone()),
        });
    }

    Ok(providers)
}

fn is_framework_path_referenced(
    index: &AttributeIndex,
    drafts: &[SingletonDraft],
    path: &CanonicalPath,
) -> bool {
    drafts
        .iter()
        .any(|draft| draft_references_path(index, draft, path))
}

fn draft_references_path(
    index: &AttributeIndex,
    draft: &SingletonDraft,
    path: &CanonicalPath,
) -> bool {
    let ConstructionSource::Constructor(constructor) = &draft.construction else {
        return false;
    };

    constructor.signature().inputs.iter().any(|input| {
        let FnArg::Typed(pattern_type) = input else {
            return false;
        };

        matches!(
            peel_target(&pattern_type.ty),
            Some(RawTarget::Single(written))
                if index.resolve_item_path(draft.item, &written).as_ref() == Some(path)
        )
    })
}

pub(crate) fn build_plan(
    index: &AttributeIndex,
    registry: &ConsoleArgumentRegistry,
    framework_provided: &[CanonicalPath],
    pool: &TagPool,
) -> Result<ContainerPlan, ContainerError> {
    let DraftedProviders {
        drafts,
        mut provided_keys,
    } = build_drafts(index)?;

    let framework_providers =
        resolve_framework_providers(index, &drafts, &mut provided_keys, framework_provided)?;

    let mut collections = CollectionTable::new();
    let mut providers = BTreeMap::new();

    for draft in drafts {
        let SingletonDraft {
            collection,
            concrete_path,
            construction,
            field_name,
            item,
            provided,
        } = draft;

        let provider_key = provided.key().clone();

        if let Some(trait_path) = collection {
            collections.add(trait_path, provider_key.clone());
        }

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

    for provider in framework_providers {
        providers.insert(provider.provided.key().clone(), provider);
    }

    Ok(ContainerPlan {
        collections,
        providers,
    })
}
