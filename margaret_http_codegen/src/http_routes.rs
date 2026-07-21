use std::collections::HashMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_request_binding_codegen::route_parameter_binders::route_parameter_binders;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;

pub(crate) fn http_routes(
    index: &AttributeIndex,
    middleware_plans: &[MiddlewarePlan],
) -> Result<HttpRouteTable, HttpCodegenError> {
    let selector = AttributeSelector::from_marker("responds_to_http");
    let binders = route_parameter_binders(index)?;
    let mut table = HttpRouteTable::new();
    let mut seen_names: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        };

        let responder = item.canonical_path().to_string();
        let HttpResponderArguments {
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
            handler_method.signature(),
            &BindingContext::Responder {
                route_path: &route_path,
                server: &server,
                subject: &subject,
            },
            &binders,
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
                layers,
                method,
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
