use std::collections::HashMap;

use quote::ToTokens;
use quote::format_ident;
use syn::GenericArgument;
use syn::PathArguments;
use syn::ReturnType;
use syn::Signature;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::format_path::format_path;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_container::is_managed::is_managed;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::binding_context::BindingContext;
use crate::binding_registries::BindingRegistries;
use crate::classify_parameters::classify_parameters;
use crate::injects_routes::injects_routes;
use crate::injects_views::injects_views;
use crate::request_binding_error::RequestBindingError;

struct InferenceReturn<'signature> {
    error: &'signature Type,
    outcome: &'signature Type,
}

fn authenticated_user_outcome_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_identity".to_string(),
        "authenticated_user_outcome".to_string(),
        "AuthenticatedUserOutcome".to_string(),
    ])
}

fn responds_to_inference_failure_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_identity".to_string(),
        "responds_to_inference_failure".to_string(),
        "RespondsToInferenceFailure".to_string(),
    ])
}

fn written(declared: &Type) -> String {
    declared.to_token_stream().to_string()
}

fn generic_type(declared: &Type, position: usize) -> Option<&Type> {
    let Type::Path(type_path) = declared else {
        return None;
    };
    let Some(PathArguments::AngleBracketed(arguments)) = type_path
        .path
        .segments
        .last()
        .map(|segment| &segment.arguments)
    else {
        return None;
    };

    match arguments.args.iter().nth(position) {
        Some(GenericArgument::Type(inner)) => Some(inner),
        _ => None,
    }
}

fn is_named(declared: &Type, name: &str) -> bool {
    let Type::Path(type_path) = declared else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == name)
}

fn inference_return(signature: &Signature) -> Option<InferenceReturn<'_>> {
    let ReturnType::Type(_, return_type) = &signature.output else {
        return None;
    };

    if !is_named(return_type, "Result") {
        return None;
    }

    Some(InferenceReturn {
        error: generic_type(return_type, 1)?,
        outcome: generic_type(return_type, 0)?,
    })
}

fn implements(index: &AttributeIndex, item: &IndexedItem, trait_path: &CanonicalPath) -> bool {
    item.trait_impls().iter().any(|trait_impl| {
        index
            .resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())
            .as_ref()
            == Some(trait_path)
    })
}

fn infer_from_request_method<'index>(
    item: &'index IndexedItem,
    provider: &str,
) -> Result<&'index IndexedMethod, RequestBindingError> {
    let selector = AttributeSelector::from_marker("infer_from_request");
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| {
            method
                .attributes()
                .iter()
                .any(|attribute| selector.matches(attribute.path()))
        })
        .collect();

    if found.len() > 1 {
        return Err(RequestBindingError::AmbiguousInferFromRequest {
            provider: provider.to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    found
        .pop()
        .ok_or_else(|| RequestBindingError::MissingInferFromRequest {
            provider: provider.to_string(),
        })
}

fn verify_outcome(
    index: &AttributeIndex,
    item: &IndexedItem,
    provider: &str,
    model: &CanonicalPath,
    outcome: &Type,
) -> Result<(), RequestBindingError> {
    if index.resolve_item_type(item, outcome).as_ref() != Some(&authenticated_user_outcome_path()) {
        return Err(RequestBindingError::InferenceReturnTypeMismatch {
            provider: provider.to_string(),
            written: written(outcome),
        });
    }

    let inferred = generic_type(outcome, 0).and_then(|inner| index.resolve_item_type(item, inner));

    if inferred.as_ref() == Some(model) {
        Ok(())
    } else {
        Err(RequestBindingError::InferredUserModelMismatch {
            provider: provider.to_string(),
            model: model.to_string(),
            written: written(outcome),
        })
    }
}

fn verify_error(
    index: &AttributeIndex,
    item: &IndexedItem,
    provider: &str,
    error: &Type,
) -> Result<(), RequestBindingError> {
    let unresolvable = || RequestBindingError::InferenceErrorUnresolvable {
        provider: provider.to_string(),
        written: written(error),
    };
    let error_path = index
        .resolve_item_type(item, error)
        .ok_or_else(unresolvable)?;
    let error_item = index
        .items()
        .iter()
        .find(|candidate| candidate.canonical_path() == &error_path)
        .ok_or_else(unresolvable)?;

    if implements(index, error_item, &responds_to_inference_failure_path()) {
        Ok(())
    } else {
        Err(RequestBindingError::InferenceErrorMissingConversion {
            provider: provider.to_string(),
            error: error_path.to_string(),
        })
    }
}

pub fn authenticated_user_providers(
    index: &AttributeIndex,
    registries: &BindingRegistries,
) -> Result<HashMap<CanonicalPath, AuthenticatedUserProvider>, RequestBindingError> {
    let selector = AttributeSelector::from_marker("infers_authenticated_user");
    let mut registry: HashMap<CanonicalPath, AuthenticatedUserProvider> = HashMap::new();

    for item in index.items() {
        let Some(matched) = AttributeQuery::new(item).find(&selector)? else {
            continue;
        };

        let concrete = item.canonical_path().clone();
        let provider = concrete.to_string();

        let Some(identifier) = index.struct_identifier(&concrete) else {
            return Err(RequestBindingError::AuthenticatedUserProviderNotAStruct { provider });
        };

        if !is_managed(item) {
            return Err(RequestBindingError::AuthenticatedUserProviderNotManaged { provider });
        }

        let Some(declared_model) = matched.args()?.path("user_model")? else {
            return Err(
                RequestBindingError::AuthenticatedUserProviderMissingUserModel { provider },
            );
        };
        let model = index
            .resolve_item_path(item, &declared_model)
            .filter(|resolved| index.struct_identifier(resolved).is_some())
            .ok_or_else(
                || RequestBindingError::AuthenticatedUserProviderUnknownUserModel {
                    provider: provider.clone(),
                    written: format_path(&declared_model),
                },
            )?;

        let method = infer_from_request_method(item, &provider)?;
        let signature = method.signature();
        let Some(InferenceReturn { error, outcome }) = inference_return(signature) else {
            return Err(RequestBindingError::InferenceReturnTypeMismatch {
                provider,
                written: signature.output.to_token_stream().to_string(),
            });
        };

        verify_outcome(index, item, &provider, &model, outcome)?;
        verify_error(index, item, &provider, error)?;

        let subject = format!("authenticated user provider '{provider}'");
        let parameters = classify_parameters(
            index,
            item,
            signature,
            &BindingContext::AuthenticatedUserProvider { subject: &subject },
            registries,
        )?;

        if let Some(existing) = registry.get(&model) {
            return Err(RequestBindingError::AmbiguousAuthenticatedUserProvider {
                model: model.to_string(),
                first: existing.application.concrete.to_string(),
                second: provider,
            });
        }

        let application = AuthenticatedUserApplication {
            concrete,
            field: identifier.field().to_string(),
            injects_routes: injects_routes(&parameters),
            injects_views: injects_views(&parameters),
            model: model.clone(),
            wrapper: format_ident!("{}", identifier.type_name()),
        };
        let method_name = format_ident!("{}", method.identifier());

        registry.insert(
            model,
            AuthenticatedUserProvider {
                application,
                method_name,
                parameters,
            },
        );
    }

    Ok(registry)
}
