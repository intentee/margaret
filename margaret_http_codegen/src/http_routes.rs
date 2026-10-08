use std::num::NonZeroU64;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_request_binding_codegen::responder_content::ResponderContent;
use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::declared_route::DeclaredRoute;
use crate::declared_routes::DeclaredRoutes;
use crate::http_codegen_error::HttpCodegenError;
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
            None => Ok(RouteContent::Unread { method }),
            Some(_) => Err(HttpCodegenError::UnusedBodyLimit {
                responder: responder.to_string(),
            }),
        },
        ResponderContent::Read(binding) => match ContentMethod::of(method) {
            None => Err(HttpCodegenError::ContentOnGetRoute {
                responder: responder.to_string(),
            }),
            Some(content_method) => match max_body_bytes {
                None => Err(HttpCodegenError::MissingBodyLimit {
                    responder: responder.to_string(),
                }),
                Some(limit) => Ok(RouteContent::Read {
                    binding,
                    limit,
                    method: content_method,
                }),
            },
        },
    }
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    routes: DeclaredRoutes,
    middleware_plans: &MiddlewarePlans,
    registries: &BindingRegistries,
) -> Result<HttpRouteTable, HttpCodegenError> {
    let mut table = HttpRouteTable::new();

    for DeclaredRoute {
        identifier,
        item,
        max_body_bytes,
        method,
        name,
        path,
        server,
    } in routes.routes
    {
        let responder = item.canonical_path().to_string();
        let subject = format!("responder '{responder}'");
        let layers = resolve_layers(item, middleware_plans, &subject)?;
        let handler_method = process_method(item)?;
        let arguments = classify_parameters(
            index,
            item,
            handler_method,
            &BindingContext::Responder {
                route_path: &path,
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

        table.insert(
            path,
            HttpRoute {
                content,
                is_async: handler_method.signature().asyncness.is_some(),
                layers,
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
