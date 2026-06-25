use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::join_candidates::join_candidates;
use margaret_attributes::resolution::Resolution;
use margaret_attributes::resolve_unique::resolve_unique;

use crate::collection_table::CollectionTable;
use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::path_text::path_text;
use crate::peel_target::peel_target;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provider_construction::ProviderConstruction;
use crate::raw_target::RawTarget;
use crate::resolve_construction::resolve_construction;
use crate::type_text::type_text;

struct ProviderDraft<'index> {
    collection: Option<CanonicalPath>,
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    provided: ProvidedType,
}

pub(crate) fn build_plan(index: &AttributeIndex) -> Result<ContainerPlan, ContainerError> {
    let trait_paths = trait_paths(index);
    let drafts = build_drafts(index, &trait_paths)?;
    let provider_keys: Vec<CanonicalPath> = drafts
        .iter()
        .map(|draft| draft.provided.key().clone())
        .collect();

    let mut collections = CollectionTable::new();
    let mut providers = Vec::new();

    for draft in drafts {
        let ProviderDraft {
            collection,
            concrete_path,
            construction,
            field_name,
            provided,
        } = draft;

        if let Some(trait_path) = collection {
            collections.add(trait_path, provided.key().clone());
        }

        let construction = match construction {
            ConstructionSource::Constructor(constructor) => {
                let dependencies = resolve_dependencies(
                    &concrete_path,
                    constructor,
                    &provider_keys,
                    &trait_paths,
                )?;

                ProviderConstruction::Constructor {
                    method: constructor.identifier().to_string(),
                    dependencies,
                }
            }
            ConstructionSource::Fieldless(shape) => ProviderConstruction::Fieldless { shape },
        };

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

fn build_drafts<'index>(
    index: &'index AttributeIndex,
    trait_paths: &[CanonicalPath],
) -> Result<Vec<ProviderDraft<'index>>, ContainerError> {
    let mut drafts: Vec<ProviderDraft> = Vec::new();

    for matched in index.select(&singleton_selector()) {
        let item = matched.item();
        let ItemKind::Struct(shape) = item.kind() else {
            return Err(ContainerError::NotASingletonStruct {
                path: item.canonical_path().to_string(),
            });
        };

        let concrete_path = item.canonical_path().clone();
        let arguments = matched.args()?;

        let provided = match arguments.path("provides")? {
            Some(written) => match resolve_unique(&written, trait_paths) {
                Resolution::Resolved(path) => ProvidedType::Interface(path),
                Resolution::NotFound => {
                    return Err(ContainerError::ProvidesUnresolvable {
                        singleton: concrete_path.to_string(),
                        written: path_text(&written),
                    });
                }
                Resolution::Ambiguous(candidates) => {
                    return Err(ContainerError::ProvidesAmbiguous {
                        singleton: concrete_path.to_string(),
                        written: path_text(&written),
                        candidates: join_candidates(&candidates),
                    });
                }
            },
            None => ProvidedType::Concrete(concrete_path.clone()),
        };

        if let Some(existing) = drafts
            .iter()
            .find(|draft| draft.provided.key() == provided.key())
        {
            return Err(ContainerError::DuplicateProvider {
                provided: provided.key().to_string(),
                first: existing.concrete_path.to_string(),
                second: concrete_path.to_string(),
            });
        }

        let field_name = provided.key().field_name();

        if let Some(existing) = drafts.iter().find(|draft| draft.field_name == field_name) {
            return Err(ContainerError::DuplicateFieldName {
                field: field_name,
                first: existing.concrete_path.to_string(),
                second: concrete_path.to_string(),
            });
        }

        let construction = resolve_construction(item.methods(), &concrete_path, shape)?;

        let collection = match arguments.path("collection")? {
            Some(written) => match resolve_unique(&written, trait_paths) {
                Resolution::Resolved(path) => Some(path),
                Resolution::NotFound => {
                    return Err(ContainerError::CollectionUnresolvable {
                        singleton: concrete_path.to_string(),
                        written: path_text(&written),
                    });
                }
                Resolution::Ambiguous(candidates) => {
                    return Err(ContainerError::CollectionAmbiguous {
                        singleton: concrete_path.to_string(),
                        written: path_text(&written),
                        candidates: join_candidates(&candidates),
                    });
                }
            },
            None => None,
        };

        drafts.push(ProviderDraft {
            collection,
            concrete_path,
            construction,
            field_name,
            provided,
        });
    }

    Ok(drafts)
}

fn resolve_dependencies(
    concrete_path: &CanonicalPath,
    constructor: &IndexedMethod,
    provider_keys: &[CanonicalPath],
    trait_paths: &[CanonicalPath],
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
            provider_keys,
            trait_paths,
        )?);
    }

    Ok(dependencies)
}

fn resolve_target(
    target: RawTarget,
    concrete_path: &CanonicalPath,
    parameter: &str,
    provider_keys: &[CanonicalPath],
    trait_paths: &[CanonicalPath],
) -> Result<DependencyKind, ContainerError> {
    match target {
        RawTarget::Single(written) => {
            let provider_key =
                resolve_reference(&written, provider_keys, concrete_path, parameter)?;

            Ok(DependencyKind::Single { provider_key })
        }
        RawTarget::Collection(written) => {
            let trait_path = resolve_reference(&written, trait_paths, concrete_path, parameter)?;

            Ok(DependencyKind::Collection { trait_path })
        }
    }
}

fn resolve_reference(
    written: &Path,
    candidates: &[CanonicalPath],
    concrete_path: &CanonicalPath,
    parameter: &str,
) -> Result<CanonicalPath, ContainerError> {
    match resolve_unique(written, candidates) {
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

fn trait_paths(index: &AttributeIndex) -> Vec<CanonicalPath> {
    index
        .items()
        .iter()
        .filter(|item| item.kind() == ItemKind::Trait)
        .map(|item| item.canonical_path().clone())
        .collect()
}

fn singleton_selector() -> AttributeSelector {
    AttributeSelector::parse("singleton").expect("the singleton selector is valid")
}
