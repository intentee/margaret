use std::collections::HashMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_container::container_bindings::ContainerBindings;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_root::binding_root;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_route_parameter_codegen::route_path::RoutePath;
use margaret_server_codegen::server_name::ServerName;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;

fn resolve_route_providers(
    arguments: &[BoundParameter],
    layers: &[LayerApplication],
    responder_path: &CanonicalPath,
    bindings: &ContainerBindings,
) -> Result<(), HttpCodegenError> {
    bindings.provider_serve_inputs(responder_path)?;

    for layer in layers {
        bindings.provider_serve_inputs(&layer.concrete)?;
    }

    for argument in arguments {
        if let Some(root) = binding_root(&argument.binding) {
            bindings.provider_serve_inputs(root)?;
        }
    }

    Ok(())
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    middleware_plans: &[MiddlewarePlan],
    registries: &BindingRegistries,
    bindings: &ContainerBindings,
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
            method,
            name,
            path,
            server,
        } = HttpResponderArguments::parse(matched.args()?, &responder)?;

        let server =
            ServerName::parse(server).map_err(|source| HttpCodegenError::InvalidServerName {
                responder: responder.clone(),
                source,
            })?;

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
                server: server.as_str(),
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

        resolve_route_providers(&arguments, &layers, item.canonical_path(), bindings)?;

        table.insert(
            route_path,
            HttpRoute {
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
