use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use syn::FnArg;
use syn::Pat;
use syn::Path;

use crate::collection_table::CollectionTable;
use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::find_constructor::find_constructor;
use crate::path_text::path_text;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::raw_target::RawTarget;
use crate::raw_target::peel_target;
use crate::resolution::Resolution;
use crate::resolution::join_candidates;
use crate::resolution::resolve_unique;
use crate::type_text::type_text;

pub(crate) struct ContainerPlan {
    pub(crate) collections: CollectionTable,
    pub(crate) providers: Vec<Provider>,
}

struct Membership {
    member_key: CanonicalPath,
    trait_path: CanonicalPath,
}

struct ProviderDraft<'index> {
    concrete_path: CanonicalPath,
    constructor: &'index IndexedMethod,
    field_name: String,
    provided: ProvidedType,
}

struct DraftSet<'index> {
    drafts: Vec<ProviderDraft<'index>>,
    memberships: Vec<Membership>,
}

pub(crate) fn build_plan(index: &AttributeIndex) -> Result<ContainerPlan, ContainerError> {
    let trait_paths = trait_paths(index);
    let DraftSet {
        drafts,
        memberships,
    } = build_drafts(index, &trait_paths)?;
    let provider_keys: Vec<CanonicalPath> = drafts
        .iter()
        .map(|draft| draft.provided.key().clone())
        .collect();

    let mut collections = CollectionTable::new();
    for membership in memberships {
        collections.add(membership.trait_path, membership.member_key);
    }

    let mut providers = Vec::new();
    for draft in drafts {
        let dependencies = resolve_dependencies(&draft, &provider_keys, &trait_paths)?;

        providers.push(Provider {
            concrete_path: draft.concrete_path,
            constructor_method: draft.constructor.identifier().to_string(),
            dependencies,
            field_name: draft.field_name,
            provided: draft.provided,
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
) -> Result<DraftSet<'index>, ContainerError> {
    let mut drafts: Vec<ProviderDraft> = Vec::new();
    let mut memberships: Vec<Membership> = Vec::new();

    for matched in index.select(&singleton_selector()) {
        let item = match matched.holder() {
            AttributeHolder::Item(item) if item.kind() == ItemKind::Struct => item,
            holder => {
                return Err(ContainerError::NotASingletonStruct {
                    path: holder.target_path(),
                });
            }
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

        if let Some(written) = arguments.path("collection")? {
            let trait_path = match resolve_unique(&written, trait_paths) {
                Resolution::Resolved(path) => path,
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
            };

            memberships.push(Membership {
                member_key: provided.key().clone(),
                trait_path,
            });
        }

        let constructor = find_constructor(index.holders(), &concrete_path)?;

        drafts.push(ProviderDraft {
            concrete_path,
            constructor,
            field_name,
            provided,
        });
    }

    Ok(DraftSet {
        drafts,
        memberships,
    })
}

fn resolve_dependencies(
    draft: &ProviderDraft,
    provider_keys: &[CanonicalPath],
    trait_paths: &[CanonicalPath],
) -> Result<Vec<DependencyKind>, ContainerError> {
    let mut dependencies = Vec::new();

    for (position, input) in draft.constructor.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: draft.concrete_path.to_string(),
                parameter: "self".to_string(),
                written: "self".to_string(),
            });
        };

        let parameter = parameter_name(&pattern_type.pat, position);

        let Some(target) = peel_target(&pattern_type.ty) else {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: draft.concrete_path.to_string(),
                parameter,
                written: type_text(&pattern_type.ty),
            });
        };

        dependencies.push(resolve_target(
            target,
            &draft.concrete_path,
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
        .holders()
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Item(item) if item.kind() == ItemKind::Trait => {
                Some(item.canonical_path().clone())
            }
            _ => None,
        })
        .collect()
}

fn singleton_selector() -> AttributeSelector {
    AttributeSelector::parse("singleton").expect("the singleton selector is valid")
}
