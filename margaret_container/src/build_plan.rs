use std::collections::BTreeMap;
use std::collections::HashMap;

use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::name_allocator::NameAllocator;

use crate::collection_table::CollectionTable;
use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::managed_arguments::ManagedArguments;
use crate::path_text::path_text;
use crate::peel_target::peel_target;
use crate::provided_singleton::ProvidedSingleton;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
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
    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for selector in managed_selectors() {
        for matched in index.select(&selector) {
            let draft = build_draft(&matched, index)?;

            if let Some(first) =
                provided_keys.insert(draft.provided.key().clone(), draft.concrete_path.clone())
            {
                return Err(ContainerError::DuplicateProvider {
                    provided: draft.provided.key().to_string(),
                    first: first.to_string(),
                    second: draft.concrete_path.to_string(),
                });
            }

            drafts.push(draft);
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
) -> Result<DirectConstruction, ContainerError> {
    match source {
        ConstructionSource::Constructor(constructor) => {
            let dependencies =
                resolve_dependencies(index, item, concrete_path, constructor, provided_keys)?;

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

        let parameter = parameter_name(&pattern_type.pat, position);

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

pub(crate) fn build_plan(
    index: &AttributeIndex,
    provided: &[ProvidedSingleton],
) -> Result<ContainerPlan, ContainerError> {
    let DraftedProviders {
        drafts,
        mut provided_keys,
    } = build_drafts(index)?;

    for ProvidedSingleton { path, .. } in provided {
        if let Some(first) = provided_keys.insert(path.clone(), path.clone()) {
            return Err(ContainerError::DuplicateProvider {
                provided: path.to_string(),
                first: first.to_string(),
                second: path.to_string(),
            });
        }
    }

    let mut collections = CollectionTable::new();
    let mut providers = BTreeMap::new();

    for draft in drafts {
        let SingletonDraft {
            collection,
            concrete_path,
            construction,
            field_name,
            item,
            provided: provided_type,
        } = draft;

        let provider_key = provided_type.key().clone();

        if let Some(trait_path) = collection {
            collections.add(trait_path, provider_key.clone());
        }

        let construction =
            resolve_direct(index, item, construction, &concrete_path, &provided_keys)?;

        providers.insert(
            provider_key,
            Provider {
                concrete_path,
                construction,
                field_name,
                provided: provided_type,
            },
        );
    }

    let mut allocator = NameAllocator::new();

    for provider in providers.values() {
        allocator.reserve(&provider.field_name);
    }

    for ProvidedSingleton { construction, path } in provided {
        let field_name = allocator.allocate(&path.field_name()).field().to_string();

        providers.insert(
            path.clone(),
            Provider {
                concrete_path: path.clone(),
                construction: DirectConstruction::Provided {
                    expression: construction.clone(),
                },
                field_name,
                provided: ProvidedType::Concrete(path.clone()),
            },
        );
    }

    Ok(ContainerPlan {
        collections,
        providers,
    })
}
