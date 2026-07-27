use std::collections::HashMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_container::is_singleton::is_singleton;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::access_policy_binding::AccessPolicyBinding;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;

fn public_access_path() -> margaret_attributes::canonical_path::CanonicalPath {
    margaret_attributes::canonical_path::CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "http".to_string(),
        "public_access".to_string(),
        "PublicAccess".to_string(),
    ])
}

fn written_path(path: &syn::Path) -> margaret_attributes::canonical_path::CanonicalPath {
    margaret_attributes::canonical_path::CanonicalPath::new(
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect(),
    )
}

fn access_policy(
    index: &AttributeIndex,
    item: &margaret_attributes::indexed_item::IndexedItem,
    written: &syn::Path,
    responder: &str,
) -> Result<AccessPolicyBinding, HttpCodegenError> {
    if written_path(written) == public_access_path() {
        return Ok(AccessPolicyBinding::Public);
    }

    let (policy, policy_item) = index
        .resolve_item_path(item, written)
        .and_then(|policy| index.item(&policy).map(|policy_item| (policy, policy_item)))
        .ok_or_else(|| HttpCodegenError::UnresolvedAccessPolicy {
            responder: responder.to_string(),
        })?;
    let Some(identifier) = index.struct_identifier(&policy) else {
        return Err(HttpCodegenError::AccessPolicyNotAStruct {
            policy: policy.to_string(),
            responder: responder.to_string(),
        });
    };

    if !is_singleton(policy_item) {
        return Err(HttpCodegenError::AccessPolicyNotSingleton {
            policy: policy.to_string(),
            responder: responder.to_string(),
        });
    }

    Ok(AccessPolicyBinding::Singleton {
        field: format_ident!("{}", identifier.field()),
        path: policy,
    })
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    middleware_plans: &[MiddlewarePlan],
    registries: &BindingRegistries,
) -> Result<HttpRouteTable, HttpCodegenError> {
    let mut table = HttpRouteTable::new();
    let mut seen_names: HashMap<String, String> = HashMap::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::RespondsToHttp) {
        let item = matched.item();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        };

        let responder = item.canonical_path().to_string();
        let HttpResponderArguments {
            access,
            method,
            name,
            path,
            server,
        } = HttpResponderArguments::parse(matched.args()?, &responder)?;
        let access_policy = access_policy(index, item, &access, &responder)?;

        if !is_snake_case_identifier(&server) {
            return Err(HttpCodegenError::InvalidServerName {
                responder: responder.clone(),
                server: server.clone(),
            });
        }

        let subject = format!("responder '{responder}'");
        let layers = resolve_layers(item, middleware_plans, &subject)?;
        let route_path =
            RoutePath::parse(&path).map_err(|source| HttpCodegenError::InsecureRoutePath {
                responder: responder.clone(),
                path: path.clone(),
                source,
            })?;
        let handler_method = process_method(item)?;
        let arguments = classify_parameters(
            index,
            item,
            handler_method,
            &BindingContext::Responder {
                route_path: &route_path,
                server: &server,
                subject: &subject,
            },
            registries,
        )?;

        if let Some(name) = &name {
            if !is_snake_case_identifier(name) {
                return Err(HttpCodegenError::InvalidRouteName {
                    name: name.clone(),
                    responder: responder.clone(),
                });
            }

            if let Some(first) = seen_names.get(name) {
                return Err(HttpCodegenError::DuplicateRouteName {
                    name: name.clone(),
                    first: first.clone(),
                    second: responder.clone(),
                });
            }

            seen_names.insert(name.clone(), responder.clone());
        }

        table.insert(
            route_path,
            HttpRoute {
                access_policy,
                layers,
                method,
                method_name: format_ident!("{}", handler_method.identifier()),
                name,
                responder_field: format_ident!("{}", identifier.field()),
                responder_path: item.canonical_path().clone(),
                arguments,
                server,
            },
        )?;
    }

    Ok(table)
}
