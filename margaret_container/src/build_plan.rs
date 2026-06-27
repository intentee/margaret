use syn::FnArg;
use syn::Pat;
use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::join_candidates::join_candidates;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::resolution::Resolution;
use margaret_attributes::resolve_unique::resolve_unique;

use crate::collection_table::CollectionTable;
use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::path_text::path_text;
use crate::peel_target::peel_target;
use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provider_construction::ProviderConstruction;
use crate::raw_target::RawTarget;
use crate::resolve_construction::resolve_construction;
use crate::type_text::type_text;

enum DraftConstruction<'index> {
    Direct(ConstructionSource<'index>),
    Factory {
        factory: &'index IndexedMethod,
        own: ConstructionSource<'index>,
    },
}

struct ProviderDraft<'index> {
    collection: Option<CanonicalPath>,
    concrete_path: CanonicalPath,
    construction: DraftConstruction<'index>,
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
            DraftConstruction::Direct(source) => ProviderConstruction::Direct(resolve_direct(
                source,
                &concrete_path,
                &provider_keys,
                &trait_paths,
            )?),
            DraftConstruction::Factory { factory, own } => ProviderConstruction::Factory {
                factory_is_async: factory.signature().asyncness.is_some(),
                factory_method: factory.identifier().to_string(),
                provider: resolve_direct(own, &concrete_path, &provider_keys, &trait_paths)?,
            },
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

#[derive(Clone, Copy)]
enum Role {
    Provider,
    Singleton,
}

fn build_drafts<'index>(
    index: &'index AttributeIndex,
    trait_paths: &[CanonicalPath],
) -> Result<Vec<ProviderDraft<'index>>, ContainerError> {
    let mut drafts: Vec<ProviderDraft> = Vec::new();

    for matched in index.select(&singleton_selector()) {
        let draft = build_draft(&matched, trait_paths, &drafts, Role::Singleton)?;

        drafts.push(draft);
    }

    for matched in index.select(&provider_selector()) {
        let draft = build_draft(&matched, trait_paths, &drafts, Role::Provider)?;

        drafts.push(draft);
    }

    for matched in index.select(&service_selector()) {
        let draft = build_draft(&matched, trait_paths, &drafts, Role::Singleton)?;

        drafts.push(draft);
    }

    for matched in index.select(&ticker_selector()) {
        let draft = build_draft(&matched, trait_paths, &drafts, Role::Singleton)?;

        drafts.push(draft);
    }

    Ok(drafts)
}

fn build_draft<'index>(
    matched: &MatchedAttribute<'index>,
    trait_paths: &[CanonicalPath],
    drafts: &[ProviderDraft<'index>],
    role: Role,
) -> Result<ProviderDraft<'index>, ContainerError> {
    let item = matched.item();
    let ItemKind::Struct(shape) = item.kind() else {
        return Err(not_a_struct(role, item.canonical_path().to_string()));
    };

    let concrete_path = item.canonical_path().clone();
    let arguments = matched.args()?;
    let provided = resolve_provided(&arguments, trait_paths, &concrete_path, role)?;

    check_unique_provided(drafts, &provided, &concrete_path)?;

    let field_name = field_name(drafts, &provided, &concrete_path)?;
    let own = resolve_construction(item.methods(), &concrete_path, shape)?;
    let collection = resolve_collection(&arguments, trait_paths, &concrete_path)?;
    let construction = match role {
        Role::Singleton => DraftConstruction::Direct(own),
        Role::Provider => DraftConstruction::Factory {
            factory: resolve_factory(item.methods(), &concrete_path)?,
            own,
        },
    };

    Ok(ProviderDraft {
        collection,
        concrete_path,
        construction,
        field_name,
        provided,
    })
}

fn not_a_struct(role: Role, path: String) -> ContainerError {
    match role {
        Role::Singleton => ContainerError::NotASingletonStruct { path },
        Role::Provider => ContainerError::ProviderNotAStruct { path },
    }
}

fn resolve_provided(
    arguments: &AttributeArgs,
    trait_paths: &[CanonicalPath],
    concrete_path: &CanonicalPath,
    role: Role,
) -> Result<ProvidedType, ContainerError> {
    match arguments.path("provides")? {
        Some(written) => resolve_interface(&written, trait_paths, concrete_path),
        None => match role {
            Role::Singleton => Ok(ProvidedType::Concrete(concrete_path.clone())),
            Role::Provider => Err(ContainerError::ProviderMissingProvides {
                provider: concrete_path.to_string(),
            }),
        },
    }
}

fn resolve_interface(
    written: &Path,
    trait_paths: &[CanonicalPath],
    concrete_path: &CanonicalPath,
) -> Result<ProvidedType, ContainerError> {
    match resolve_unique(written, trait_paths, crate_root(concrete_path)) {
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
    arguments: &AttributeArgs,
    trait_paths: &[CanonicalPath],
    concrete_path: &CanonicalPath,
) -> Result<Option<CanonicalPath>, ContainerError> {
    match arguments.path("collection")? {
        Some(written) => match resolve_unique(&written, trait_paths, crate_root(concrete_path)) {
            Resolution::Resolved(path) => Ok(Some(path)),
            Resolution::NotFound => Err(ContainerError::CollectionUnresolvable {
                singleton: concrete_path.to_string(),
                written: path_text(&written),
            }),
            Resolution::Ambiguous(candidates) => Err(ContainerError::CollectionAmbiguous {
                singleton: concrete_path.to_string(),
                written: path_text(&written),
                candidates: join_candidates(&candidates),
            }),
        },
        None => Ok(None),
    }
}

fn check_unique_provided(
    drafts: &[ProviderDraft],
    provided: &ProvidedType,
    concrete_path: &CanonicalPath,
) -> Result<(), ContainerError> {
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

    Ok(())
}

fn field_name(
    drafts: &[ProviderDraft],
    provided: &ProvidedType,
    concrete_path: &CanonicalPath,
) -> Result<String, ContainerError> {
    let field_name = provided.key().field_name();

    if let Some(existing) = drafts.iter().find(|draft| draft.field_name == field_name) {
        return Err(ContainerError::DuplicateFieldName {
            field: field_name,
            first: existing.concrete_path.to_string(),
            second: concrete_path.to_string(),
        });
    }

    Ok(field_name)
}

fn resolve_factory<'index>(
    methods: &'index [IndexedMethod],
    concrete_path: &CanonicalPath,
) -> Result<&'index IndexedMethod, ContainerError> {
    let mut found: Vec<&IndexedMethod> = methods
        .iter()
        .filter(|method| has_provide_attribute(method))
        .collect();

    if found.len() > 1 {
        return Err(ContainerError::AmbiguousProvideMethod {
            provider: concrete_path.to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    found
        .pop()
        .ok_or_else(|| ContainerError::ProviderMissingProvideMethod {
            provider: concrete_path.to_string(),
        })
}

fn has_provide_attribute(method: &IndexedMethod) -> bool {
    method.attributes().iter().any(|attribute| {
        attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "provide")
    })
}

fn resolve_direct(
    source: ConstructionSource,
    concrete_path: &CanonicalPath,
    provider_keys: &[CanonicalPath],
    trait_paths: &[CanonicalPath],
) -> Result<DirectConstruction, ContainerError> {
    match source {
        ConstructionSource::Constructor(constructor) => {
            let dependencies =
                resolve_dependencies(concrete_path, constructor, provider_keys, trait_paths)?;

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
    match resolve_unique(written, candidates, crate_root(concrete_path)) {
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

fn crate_root(path: &CanonicalPath) -> &str {
    path.segments()
        .first()
        .expect("a canonical path has at least one segment")
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

fn provider_selector() -> AttributeSelector {
    AttributeSelector::parse("provider").expect("the provider selector is valid")
}

fn service_selector() -> AttributeSelector {
    AttributeSelector::parse("service").expect("the service selector is valid")
}

fn ticker_selector() -> AttributeSelector {
    AttributeSelector::parse("ticker").expect("the ticker selector is valid")
}
