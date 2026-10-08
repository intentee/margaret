use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_base::field_base;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::indexed_struct::IndexedStruct;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_database_codegen::database_canonical_path::database_canonical_path;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
use margaret_input_weaving::input_value::InputValue;
use margaret_input_weaving::weaving_kind::WeavingKind;
use margaret_serve_input_codegen::declared_serve_inputs::DeclaredServeInputs;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_token_issuance_codegen::token_issuance_path::token_issuance_path;

use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::draft_parameter::DraftParameter;
use crate::framework_construction::FrameworkConstruction;
use crate::framework_dependency::FrameworkDependency;
use crate::framework_enablement::FrameworkEnablement;
use crate::framework_injection_role::FrameworkInjectionRole;
use crate::framework_provider::FrameworkProvider;
use crate::parameter_target::ParameterTarget;
use crate::path_text::path_text;
use crate::provider::Provider;
use crate::provider_requirement::ProviderRequirement;
use crate::resolve_construction::resolve_construction;
use crate::singleton_declaration::SingletonDeclaration;
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

fn check_declared_item(
    item: &IndexedItem,
    declaration: SingletonDeclaration,
) -> Result<(), ContainerError> {
    let attribute = declaration.attribute().name();
    let path = item.canonical_path().to_string();

    if !matches!(item.kind(), ItemKind::Struct(_)) {
        return Err(ContainerError::DeclarationNotAStruct { attribute, path });
    }

    if !item.has_framework_attribute(FrameworkAttribute::Singleton) {
        return Err(ContainerError::DeclarationRequiresSingleton { attribute, path });
    }

    Ok(())
}

fn build_drafts(index: &AttributeIndex) -> Result<DraftedContainer<'_>, ContainerError> {
    for declaration in SingletonDeclaration::ALL {
        for matched in index.select_framework_attribute(declaration.attribute()) {
            check_declared_item(matched.item(), declaration)?;
        }
    }

    let mut provider_drafts: Vec<Draft> = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::Singleton) {
        provider_drafts.push(build_provider_draft(&matched, index)?);
    }

    let mut provided_keys: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for draft in &provider_drafts {
        provided_keys.insert(draft.concrete_path.clone(), draft.concrete_path.clone());
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
    let Some(IndexedStruct { identifier, shape }) = index.indexed_struct(item) else {
        return Err(ContainerError::NotASingletonStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();

    reject_singleton_arguments(matched.args()?, &concrete_path)?;

    for declaration in SingletonDeclaration::ALL {
        if item.has_framework_attribute(declaration.attribute()) {
            check_declaration(index, item, declaration)?;
        }
    }

    let field_name = identifier.field().to_string();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let parameters = read_draft_parameters(index, item, &construction);

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name,
        parameters,
        type_name: identifier.type_name().to_string(),
    })
}

fn check_declaration(
    index: &AttributeIndex,
    item: &IndexedItem,
    declaration: SingletonDeclaration,
) -> Result<(), ContainerError> {
    let attribute = declaration.attribute().name();

    if has_concrete_role(item) {
        return Err(ContainerError::ConflictingDeclarationRole {
            attribute,
            path: item.canonical_path().to_string(),
        });
    }

    let required = declaration.required_trait();

    if implements_trait(index, item, &required) {
        Ok(())
    } else {
        Err(ContainerError::DeclarationMissingTrait {
            attribute,
            path: item.canonical_path().to_string(),
            required: required.to_string(),
        })
    }
}

fn build_construction_draft<'index>(
    item: &'index IndexedItem,
    index: &AttributeIndex,
) -> Result<Draft<'index>, ContainerError> {
    let Some(IndexedStruct { identifier, shape }) = index.indexed_struct(item) else {
        return Err(ContainerError::RoleNotAStruct {
            path: item.canonical_path().to_string(),
        });
    };

    let concrete_path = item.canonical_path().clone();
    let construction = resolve_construction(item.methods(), &concrete_path, shape)?;
    let parameters = read_draft_parameters(index, item, &construction);

    Ok(Draft {
        concrete_path: concrete_path.clone(),
        construction,
        field_name: identifier.field().to_string(),
        parameters,
        type_name: identifier.type_name().to_string(),
    })
}

fn read_draft_parameters<'index>(
    index: &AttributeIndex,
    item: &IndexedItem,
    construction: &ConstructionSource<'index>,
) -> Vec<DraftParameter<'index>> {
    let ConstructionSource::Constructor(constructor) = construction else {
        return Vec::new();
    };

    constructor
        .parameters()
        .iter()
        .map(|parameter| DraftParameter::read(index, item, parameter))
        .collect()
}

fn has_concrete_role(item: &IndexedItem) -> bool {
    concrete_roles()
        .iter()
        .any(|role| item.has_framework_attribute(*role))
}

fn implements_trait(index: &AttributeIndex, item: &IndexedItem, required: &CanonicalPath) -> bool {
    item.trait_impls().iter().any(|trait_impl| {
        index
            .resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())
            .as_ref()
            == Some(required)
    })
}

struct DependencyResolver<'resolver> {
    framework_providers: &'resolver [FrameworkProvider],
    provided_keys: &'resolver HashMap<CanonicalPath, CanonicalPath>,
    serve_inputs: &'resolver DeclaredServeInputs,
}

impl DependencyResolver<'_> {
    fn resolve_dependencies(
        &self,
        concrete_path: &CanonicalPath,
        constructor: &IndexedMethod,
        parameters: &[DraftParameter],
    ) -> Result<Vec<DependencyKind>, ContainerError> {
        let mut dependencies = Vec::new();

        if constructor.has_receiver() {
            return Err(ContainerError::UnsupportedParameterShape {
                singleton: concrete_path.to_string(),
                parameter: "self".to_string(),
                written: "self".to_string(),
            });
        }

        for DraftParameter { indexed, target } in parameters {
            let parameter = indexed.name().to_string();

            dependencies.push(
                match self.serve_inputs.input(concrete_path, indexed.position()) {
                    Some(input) => DependencyKind::ServeInput {
                        input: Box::new(input.clone()),
                    },
                    None => match target {
                        ParameterTarget::Resolved { resolved, written } => resolve_target(
                            resolved,
                            written,
                            concrete_path,
                            &parameter,
                            self.provided_keys,
                            self.framework_providers,
                        )?,
                        ParameterTarget::Unresolved { written } => {
                            return Err(missing_provider(concrete_path, &parameter, written));
                        }
                        ParameterTarget::UnsupportedShape => {
                            return Err(ContainerError::UnsupportedParameterShape {
                                singleton: concrete_path.to_string(),
                                parameter,
                                written: type_text(indexed.declared()),
                            });
                        }
                    },
                },
            );
        }

        Ok(dependencies)
    }

    fn resolve_direct(
        &self,
        source: &ConstructionSource,
        parameters: &[DraftParameter],
        concrete_path: &CanonicalPath,
    ) -> Result<DirectConstruction, ContainerError> {
        match source {
            ConstructionSource::Constructor(constructor) => {
                let dependencies =
                    self.resolve_dependencies(concrete_path, constructor, parameters)?;

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
}

fn resolve_target(
    resolved: &CanonicalPath,
    written: &Path,
    concrete_path: &CanonicalPath,
    parameter: &str,
    provided_keys: &HashMap<CanonicalPath, CanonicalPath>,
    framework_providers: &[FrameworkProvider],
) -> Result<DependencyKind, ContainerError> {
    if provided_keys.contains_key(resolved) {
        return Ok(DependencyKind::Single {
            provider_key: resolved.clone(),
        });
    }

    let restricted_role = framework_providers
        .iter()
        .find(|framework_provider| &framework_provider.provided == resolved)
        .map(|framework_provider| &framework_provider.injection);

    match restricted_role {
        Some(
            FrameworkInjectionRole::FrameworkState
            | FrameworkInjectionRole::OAuthClient(_)
            | FrameworkInjectionRole::Runner
            | FrameworkInjectionRole::TrustedIssuer(_),
        ) => Err(ContainerError::FrameworkOnlyProvider {
            parameter: parameter.to_string(),
            provider: resolved.to_string(),
            singleton: concrete_path.to_string(),
        }),
        Some(FrameworkInjectionRole::Unmarked) | None => {
            Err(missing_provider(concrete_path, parameter, written))
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

struct Draft<'index> {
    concrete_path: CanonicalPath,
    construction: ConstructionSource<'index>,
    field_name: String,
    parameters: Vec<DraftParameter<'index>>,
    type_name: String,
}

struct DraftedContainer<'index> {
    construction_drafts: Vec<Draft<'index>>,
    provided_keys: HashMap<CanonicalPath, CanonicalPath>,
    provider_drafts: Vec<Draft<'index>>,
}

fn resolve_framework_providers(
    index: &AttributeIndex,
    drafted: &mut DraftedContainer,
    framework_providers: &[FrameworkProvider],
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
) -> Result<Vec<Provider>, ContainerError> {
    let declared = declared_framework_providers(framework_providers)?;
    let included = included_framework_providers(
        &drafted.provider_drafts,
        &drafted.construction_drafts,
        framework_providers,
    );
    let mut allocator = index.reserved_allocator();
    let mut providers = Vec::new();
    let mut ordered: Vec<&FrameworkProvider> = declared
        .into_values()
        .filter(|framework_provider| included.contains(&framework_provider.provided))
        .collect();

    ordered.sort_by(|left, right| left.provided.cmp(&right.provided));

    for framework_provider in ordered {
        let path = framework_provider.provided.clone();

        if drafted.provided_keys.contains_key(&path) {
            return Err(ContainerError::AmbiguousFrameworkProvider {
                path: path.to_string(),
            });
        }

        if let FrameworkInjectionRole::Unmarked = framework_provider.injection {
            drafted.provided_keys.insert(path.clone(), path.clone());
        }

        let construction =
            resolve_framework_construction(index, framework_provider, database, token_issuance)?;
        let identifier = allocator.allocate(&field_base(&path));
        let field_name = identifier.field().to_string();
        let type_name = identifier.type_name().to_string();

        providers.push(Provider {
            concrete_path: path,
            construction,
            field_name,
            injection: framework_provider.injection.clone(),
            requirement: match framework_provider.enablement {
                FrameworkEnablement::Declared => ProviderRequirement::Declared,
                FrameworkEnablement::Dependency | FrameworkEnablement::WhenReferenced => {
                    ProviderRequirement::Optional
                }
            },
            type_name,
        });
    }

    Ok(providers)
}

fn declared_framework_providers(
    framework_providers: &[FrameworkProvider],
) -> Result<HashMap<CanonicalPath, &FrameworkProvider>, ContainerError> {
    let declared: HashMap<CanonicalPath, &FrameworkProvider> = framework_providers
        .iter()
        .map(|framework_provider| (framework_provider.provided.clone(), framework_provider))
        .collect();

    for framework_provider in framework_providers {
        if let Some(dependency) = framework_provider_dependencies(&framework_provider.construction)
            .into_iter()
            .find(|dependency| !declared.contains_key(*dependency))
        {
            return Err(ContainerError::UndeclaredFrameworkDependency {
                provider: framework_provider.provided.to_string(),
                dependency: dependency.to_string(),
            });
        }
    }

    Ok(declared)
}

fn framework_dependency_kind(
    index: &AttributeIndex,
    dependency: &FrameworkDependency,
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
    provided: &CanonicalPath,
) -> Result<DependencyKind, ContainerError> {
    match dependency {
        FrameworkDependency::Constant(path) => Ok(DependencyKind::Constant { path: path.clone() }),
        FrameworkDependency::EnvironmentVariable { name, value_type } => {
            Ok(DependencyKind::ServeInput {
                input: Box::new(ServeInput::EnvironmentVariable(EnvironmentVariable {
                    name: name.clone(),
                    value: InputValue {
                        required: true,
                        value_type: value_type.clone(),
                        weaving: WeavingKind::from_canonical(index, value_type, true),
                    },
                })),
            })
        }
        FrameworkDependency::Database { .. } => match database {
            DeclaredPostgresDatabase::Absent => Err(ContainerError::MissingPostgresDatabase {
                provider: provided.to_string(),
            }),
            DeclaredPostgresDatabase::Declared(_) => Ok(DependencyKind::Single {
                provider_key: database_canonical_path(),
            }),
        },
        FrameworkDependency::Provider(provider_key)
        | FrameworkDependency::SingletonView(provider_key) => Ok(DependencyKind::Single {
            provider_key: provider_key.clone(),
        }),
        FrameworkDependency::Providers(provider_keys) => Ok(DependencyKind::Collection {
            provider_keys: provider_keys.clone(),
        }),
        FrameworkDependency::RouteUrl(route) => Ok(DependencyKind::ServeInput {
            input: Box::new(ServeInput::RouteUrl(route.clone())),
        }),
        FrameworkDependency::Urls(sources) => Ok(DependencyKind::Urls {
            sources: sources.clone(),
        }),
        FrameworkDependency::TokenIssuance => match token_issuance {
            DeclaredTokenIssuance::Absent => Err(ContainerError::MissingTokenIssuance {
                provider: provided.to_string(),
            }),
            DeclaredTokenIssuance::Declared(_) => Ok(DependencyKind::Constant {
                path: token_issuance_path(),
            }),
        },
    }
}

fn framework_dependency_kinds(
    index: &AttributeIndex,
    dependencies: &[FrameworkDependency],
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
    provided: &CanonicalPath,
) -> Result<Vec<DependencyKind>, ContainerError> {
    dependencies
        .iter()
        .map(|dependency| {
            framework_dependency_kind(index, dependency, database, token_issuance, provided)
        })
        .collect()
}

fn resolve_framework_construction(
    index: &AttributeIndex,
    FrameworkProvider {
        construction,
        provided,
        ..
    }: &FrameworkProvider,
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
) -> Result<DirectConstruction, ContainerError> {
    Ok(match construction {
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
            outcome,
        } => DirectConstruction::FrameworkConstructor {
            dependencies: framework_dependency_kinds(
                index,
                dependencies,
                database,
                token_issuance,
                provided,
            )?,
            is_async: *is_async,
            method: method.clone(),
            outcome: *outcome,
        },
        FrameworkConstruction::Unit => DirectConstruction::FrameworkUnit,
    })
}

fn framework_provider_dependencies(construction: &FrameworkConstruction) -> Vec<&CanonicalPath> {
    match construction {
        FrameworkConstruction::Accessor { source, .. } => vec![source],
        FrameworkConstruction::Constructor { dependencies, .. } => dependencies
            .iter()
            .flat_map(|dependency| match dependency {
                FrameworkDependency::Provider(provider_key) => std::slice::from_ref(provider_key),
                FrameworkDependency::Providers(provider_keys) => provider_keys.as_slice(),
                FrameworkDependency::Constant(_)
                | FrameworkDependency::Database { .. }
                | FrameworkDependency::EnvironmentVariable { .. }
                | FrameworkDependency::RouteUrl(_)
                | FrameworkDependency::SingletonView(_)
                | FrameworkDependency::TokenIssuance
                | FrameworkDependency::Urls(_) => &[],
            })
            .collect(),
        FrameworkConstruction::Unit => Vec::new(),
    }
}

fn included_framework_providers(
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    framework_providers: &[FrameworkProvider],
) -> HashSet<CanonicalPath> {
    let mut triggered: HashSet<CanonicalPath> = HashSet::new();

    for framework_provider in framework_providers {
        if is_directly_triggered(provider_drafts, construction_drafts, framework_provider) {
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
                if triggered.insert(dependency.clone()) {
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
    provider_drafts: &[Draft],
    construction_drafts: &[Draft],
    framework_provider: &FrameworkProvider,
) -> bool {
    match framework_provider.enablement {
        FrameworkEnablement::Declared => true,
        FrameworkEnablement::Dependency => false,
        FrameworkEnablement::WhenReferenced => {
            is_framework_path_referenced(provider_drafts, &framework_provider.provided)
                || is_framework_path_referenced(construction_drafts, &framework_provider.provided)
        }
    }
}

fn is_framework_path_referenced(drafts: &[Draft], path: &CanonicalPath) -> bool {
    drafts
        .iter()
        .any(|draft| draft_references_path(draft, path))
}

fn draft_references_path(draft: &Draft, path: &CanonicalPath) -> bool {
    draft
        .parameters
        .iter()
        .any(|parameter| parameter.target.resolves_to(path))
}

pub(crate) fn build_plan(
    index: &AttributeIndex,
    serve_inputs: &DeclaredServeInputs,
    framework_providers: &[FrameworkProvider],
    database: &DeclaredPostgresDatabase,
    token_issuance: &DeclaredTokenIssuance,
) -> Result<ContainerPlan, ContainerError> {
    let mut drafted = build_drafts(index)?;
    let resolved_framework_providers = resolve_framework_providers(
        index,
        &mut drafted,
        framework_providers,
        database,
        token_issuance,
    )?;
    let DraftedContainer {
        construction_drafts,
        provided_keys,
        provider_drafts,
        ..
    } = drafted;
    let resolver = DependencyResolver {
        framework_providers,
        provided_keys: &provided_keys,
        serve_inputs,
    };

    let mut providers = BTreeMap::new();
    let mut constructions = BTreeMap::new();

    for draft in provider_drafts {
        let Draft {
            concrete_path,
            construction,
            field_name,
            parameters,
            type_name,
        } = draft;

        let provider_key = concrete_path.clone();

        let construction = resolver.resolve_direct(&construction, &parameters, &concrete_path)?;

        providers.insert(
            provider_key,
            Provider {
                concrete_path,
                construction,
                field_name,
                injection: FrameworkInjectionRole::Unmarked,
                requirement: ProviderRequirement::Singleton,
                type_name,
            },
        );
    }

    for draft in construction_drafts {
        let Draft {
            concrete_path,
            construction,
            field_name,
            parameters,
            type_name,
        } = draft;

        let construction = resolver.resolve_direct(&construction, &parameters, &concrete_path)?;

        constructions.insert(
            concrete_path.clone(),
            Provider {
                concrete_path,
                construction,
                field_name,
                injection: FrameworkInjectionRole::Unmarked,
                requirement: ProviderRequirement::Optional,
                type_name,
            },
        );
    }

    for provider in resolved_framework_providers {
        providers.insert(provider.concrete_path.clone(), provider);
    }

    let injectable = providers.keys().cloned().collect();
    let entries = providers.into_iter().chain(constructions).collect();
    let dependency_order = topological_order(&entries)?;

    ContainerPlan::new(dependency_order, entries, injectable)
}
