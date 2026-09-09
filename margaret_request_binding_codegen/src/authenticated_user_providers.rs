use std::collections::HashMap;

use quote::format_ident;

use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_container::is_singleton::is_singleton;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::binding_context::BindingContext;
use crate::binding_registries::BindingRegistries;
use crate::classified_parameters::ClassifiedParameters;
use crate::classify_parameters::classify_parameters;
use crate::infers_authenticated_user_arguments::InfersAuthenticatedUserArguments;
use crate::injects_routes::injects_routes;
use crate::injects_views::injects_views;
use crate::request_binding_error::RequestBindingError;

fn infer_from_request_method<'index>(
    item: &'index IndexedItem,
    provider: &str,
) -> Result<&'index IndexedMethod, RequestBindingError> {
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| method.has_framework_attribute(FrameworkAttribute::InferFromRequest))
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

/// # Errors
///
/// Returns `RequestBindingError::AuthenticatedUserProviderNotAStruct` or `RequestBindingError::AuthenticatedUserProviderRequiresSingleton` or `RequestBindingError::AuthenticatedUserProviderUnknownUserModel`.
pub fn authenticated_user_providers(
    index: &AttributeIndex,
    registries: &BindingRegistries,
) -> Result<HashMap<CanonicalPath, AuthenticatedUserProvider>, RequestBindingError> {
    let mut registry: HashMap<CanonicalPath, AuthenticatedUserProvider> = HashMap::new();

    for item in index.items() {
        let Some(matched) = AttributeQuery::new(item)
            .find_framework(FrameworkAttribute::InfersAuthenticatedUser)?
        else {
            continue;
        };

        let concrete = item.canonical_path().clone();
        let provider = concrete.to_string();

        let Some(identifier) = index.struct_identifier(&concrete) else {
            return Err(RequestBindingError::AuthenticatedUserProviderNotAStruct { provider });
        };

        if !is_singleton(item) {
            return Err(
                RequestBindingError::AuthenticatedUserProviderRequiresSingleton { provider },
            );
        }

        let InfersAuthenticatedUserArguments {
            user_model: declared_model,
        } = InfersAuthenticatedUserArguments::parse(matched.args()?, &provider)?;
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

        let subject = format!("authenticated user provider '{provider}'");
        let ClassifiedParameters {
            body_intake,
            parameters,
        } = classify_parameters(
            index,
            item,
            method,
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
        let is_async = method.signature().asyncness.is_some();
        let method_name = format_ident!("{}", method.identifier());

        registry.insert(
            model,
            AuthenticatedUserProvider {
                application,
                body_intake,
                is_async,
                method_name,
                parameters,
            },
        );
    }

    Ok(registry)
}
