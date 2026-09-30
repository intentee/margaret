use std::collections::HashMap;
use std::num::NonZeroU64;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_request_binding_codegen::responder_content::ResponderContent;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::route_content::RouteContent;

fn route_content(
    responder_content: ResponderContent,
    max_body_bytes: Option<NonZeroU64>,
    method: RouteMethod,
    responder: &str,
) -> Result<RouteContent, HttpCodegenError> {
    match responder_content {
        ResponderContent::Unread => match max_body_bytes {
            None => Ok(RouteContent::Unread),
            Some(_) => Err(HttpCodegenError::UnusedBodyLimit {
                responder: responder.to_string(),
            }),
        },
        ResponderContent::Read(_) if method == RouteMethod::Get => {
            Err(HttpCodegenError::ContentOnGetRoute {
                responder: responder.to_string(),
            })
        }
        ResponderContent::Read(binding) => match max_body_bytes {
            None => Err(HttpCodegenError::MissingBodyLimit {
                responder: responder.to_string(),
            }),
            Some(limit) => Ok(RouteContent::Read { binding, limit }),
        },
    }
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    middleware_plans: &MiddlewarePlans,
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
            max_body_bytes,
            method,
            name,
            path,
            server,
        } = HttpResponderArguments::parse(matched.args()?, &responder)?;

        if !is_snake_case_identifier(&server) {
            return Err(HttpCodegenError::InvalidServerName {
                responder: responder.clone(),
                server: server.clone(),
            });
        }

        let subject = format!("responder '{responder}'");
        let layers = resolve_layers(item, middleware_plans, &subject)?;
        let route_path = RoutePath::parse(&path);
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

        let content = route_content(
            ResponderContent::of(&arguments, &subject)?,
            max_body_bytes,
            method,
            &responder,
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
                content,
                is_async: handler_method.signature().asyncness.is_some(),
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
