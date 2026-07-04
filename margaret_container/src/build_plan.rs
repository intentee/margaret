use std::collections::HashMap;

use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::join_candidates::join_candidates;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::resolution::Resolution;
use margaret_attributes::resolution_index::ResolutionIndex;

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
    trait_resolution: &ResolutionIndex,
) -> Result<Vec<SingletonDraft<'index>>, ContainerError> {
    let mut drafts: Vec<SingletonDraft> = Vec::new();
    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for selector in managed_selectors() {
        for matched in index.select(&selector) {
            let draft = build_draft(&matched, index, trait_resolution, &provided_keys)?;

            provided_keys.insert(draft.provided.key().clone(), draft.concrete_path.clone());
            drafts.push(draft);
        }
    }

    Ok(drafts)
}

fn build_draft<'index>(
    matched: &MatchedAttribute<'index>,
    index: &AttributeIndex,
    trait_resolution: &ResolutionIndex,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
) -> Result<SingletonDraft<'index>, ContainerError> {
    let item = matched.item();
    let ItemKind::Struct(shape) = item.kind() else {
        return Err(ContainerError::NotASingletonStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();
    let ManagedArguments {
        collection,
        provides,
    } = ManagedArguments::parse(matched.args()?)?;
    let provided = resolve_provided(provides.as_ref(), trait_resolution, &concrete_path)?;

    check_unique_provided(provided_keys, &provided, &concrete_path)?;

    let field_name = index.field_name(&concrete_path).to_string();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let collection = resolve_collection(collection.as_ref(), trait_resolution, &concrete_path)?;

    Ok(SingletonDraft {
        collection,
        concrete_path,
        construction,
        field_name,
        provided,
    })
}

fn resolve_provided(
    provides: Option<&Path>,
    trait_resolution: &ResolutionIndex,
    concrete_path: &CanonicalPath,
) -> Result<ProvidedType, ContainerError> {
    match provides {
        Some(written) => resolve_interface(written, trait_resolution, concrete_path),
        None => Ok(ProvidedType::Concrete(concrete_path.clone())),
    }
}

fn resolve_interface(
    written: &Path,
    trait_resolution: &ResolutionIndex,
    concrete_path: &CanonicalPath,
) -> Result<ProvidedType, ContainerError> {
    match trait_resolution.resolve(written) {
        Resolution::Resolved(path) => Ok(ProvidedType::Interface(path)),
        Resolution::NotFound => Err(ContainerError::ProvidesUnresolvable {
            singleton: concrete_path.to_string(),
            written: path_text(written),
        }),
        Resolution::Ambiguous(candidates) => Err(ContainerError::ProvidesAmbiguous {
            singleton: concrete_path.to_string(),
            written: path_text(written),
            candidates: join_candidates(&candidates),
        }),
    }
}

fn resolve_collection(
    collection: Option<&Path>,
    trait_resolution: &ResolutionIndex,
    concrete_path: &CanonicalPath,
) -> Result<Option<CanonicalPath>, ContainerError> {
    match collection {
        Some(written) => match trait_resolution.resolve(written) {
            Resolution::Resolved(path) => Ok(Some(path)),
            Resolution::NotFound => Err(ContainerError::CollectionUnresolvable {
                singleton: concrete_path.to_string(),
                written: path_text(written),
            }),
            Resolution::Ambiguous(candidates) => Err(ContainerError::CollectionAmbiguous {
                singleton: concrete_path.to_string(),
                written: path_text(written),
                candidates: join_candidates(&candidates),
            }),
        },
        None => Ok(None),
    }
}

fn check_unique_provided(
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
    provided: &ProvidedType,
    concrete_path: &CanonicalPath,
) -> Result<(), ContainerError> {
    if let Some(existing) = provided_keys.get(provided.key()) {
        return Err(ContainerError::DuplicateProvider {
            provided: provided.key().to_string(),
            first: existing.to_string(),
            second: concrete_path.to_string(),
        });
    }

    Ok(())
}

fn resolve_direct(
    source: ConstructionSource,
    concrete_path: &CanonicalPath,
    provider_resolution: &ResolutionIndex,
    trait_resolution: &ResolutionIndex,
) -> Result<DirectConstruction, ContainerError> {
    match source {
        ConstructionSource::Constructor(constructor) => {
            let dependencies = resolve_dependencies(
                concrete_path,
                constructor,
                provider_resolution,
                trait_resolution,
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
    concrete_path: &CanonicalPath,
    constructor: &IndexedMethod,
    provider_resolution: &ResolutionIndex,
    trait_resolution: &ResolutionIndex,
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
            target,
            concrete_path,
            &parameter,
            provider_resolution,
            trait_resolution,
        )?);
    }

    Ok(dependencies)
}

fn resolve_target(
    target: RawTarget,
    concrete_path: &CanonicalPath,
    parameter: &str,
    provider_resolution: &ResolutionIndex,
    trait_resolution: &ResolutionIndex,
) -> Result<DependencyKind, ContainerError> {
    match target {
        RawTarget::Single(written) => {
            let provider_key =
                resolve_reference(&written, provider_resolution, concrete_path, parameter)?;

            Ok(DependencyKind::Single { provider_key })
        }
        RawTarget::Collection(written) => {
            let trait_path =
                resolve_reference(&written, trait_resolution, concrete_path, parameter)?;

            Ok(DependencyKind::Collection { trait_path })
        }
    }
}

fn resolve_reference(
    written: &Path,
    resolution: &ResolutionIndex,
    concrete_path: &CanonicalPath,
    parameter: &str,
) -> Result<CanonicalPath, ContainerError> {
    match resolution.resolve(written) {
        Resolution::Resolved(path) => Ok(path),
        Resolution::NotFound => Err(ContainerError::MissingProvider {
            singleton: concrete_path.to_string(),
            parameter: parameter.to_string(),
            written: path_text(written),
        }),
        Resolution::Ambiguous(found) => Err(ContainerError::AmbiguousReference {
            singleton: concrete_path.to_string(),
            parameter: parameter.to_string(),
            written: path_text(written),
            candidates: join_candidates(&found),
        }),
    }
}

fn parameter_name(pattern: &Pat, position: usize) -> String {
    match pattern {
        Pat::Ident(pattern_ident) => pattern_ident.ident.to_string(),
        _ => position.to_string(),
    }
}

fn singleton_selector() -> AttributeSelector {
    AttributeSelector::parse("singleton").expect("the singleton selector is valid")
}

fn service_selector() -> AttributeSelector {
    AttributeSelector::parse("service").expect("the service selector is valid")
}

fn scheduled_with_tick_timer_selector() -> AttributeSelector {
    AttributeSelector::parse("scheduled_with_tick_timer")
        .expect("the scheduled_with_tick_timer selector is valid")
}

struct SingletonDraft<'index> {
    collection: Option<CanonicalPath>,
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    provided: ProvidedType,
}

pub(crate) fn build_plan(index: &AttributeIndex) -> Result<ContainerPlan, ContainerError> {
    let trait_resolution = index.trait_resolution();
    let drafts = build_drafts(index, trait_resolution)?;
    let provider_keys: Vec<CanonicalPath> = drafts
        .iter()
        .map(|draft| draft.provided.key().clone())
        .collect();

    let provider_resolution = ResolutionIndex::new(provider_keys);
    let mut collections = CollectionTable::new();
    let mut providers = Vec::new();

    for draft in drafts {
        let SingletonDraft {
            collection,
            concrete_path,
            construction,
            field_name,
            provided,
        } = draft;

        if let Some(trait_path) = collection {
            collections.add(trait_path, provided.key().clone());
        }

        let construction = resolve_direct(
            construction,
            &concrete_path,
            &provider_resolution,
            trait_resolution,
        )?;

        providers.push(Provider {
            concrete_path,
            construction,
            field_name,
            provided,
        });
    }

    Ok(ContainerPlan {
        collections,
        providers,
    })
}
