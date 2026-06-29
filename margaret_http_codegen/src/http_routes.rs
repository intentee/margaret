use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::matched_attribute::MatchedAttribute;

use crate::build_registry::build_registry;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::layer_application::LayerApplication;
use crate::middleware_binding::MiddlewareBinding;
use crate::path_parameter_names::path_parameter_names;
use crate::registries::Registries;
use crate::responder_method::responder_method;

fn marker_value(
    marker: &MatchedAttribute,
    responder: &str,
) -> Result<TokenStream, HttpCodegenError> {
    let arguments = marker.args()?;

    if arguments.is_empty() {
        return Ok(quote! { () });
    }

    match arguments.positional(0) {
        Some(value) if arguments.positional(1).is_none() => Ok(quote! { #value }),
        _ => Err(HttpCodegenError::MalformedMarker {
            marker: marker.path(),
            responder: responder.to_string(),
        }),
    }
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    bindings: &[MiddlewareBinding],
) -> Result<Vec<HttpRoute>, HttpCodegenError> {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");
    let struct_paths = index.struct_paths();
    let binders = build_registry(
        index,
        &struct_paths,
        "route_parameter_binder",
        "Model",
        |binder| HttpCodegenError::RouteParameterBinderModel { binder },
        |model, first, second| HttpCodegenError::AmbiguousRouteParameterBinder {
            model,
            first,
            second,
        },
    )?;
    let gates = build_registry(
        index,
        &struct_paths,
        "crud_gate",
        "Subject",
        |gate| HttpCodegenError::CrudGateSubject { gate },
        |subject, first, second| HttpCodegenError::AmbiguousCrudGate {
            subject,
            first,
            second,
        },
    )?;
    let registries = Registries {
        binders: &binders,
        gates: &gates,
        struct_paths: &struct_paths,
    };
    let mut routes = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let arguments = matched.args()?;
        let responder = item.canonical_path().to_string();
        let method = arguments
            .path("method")?
            .ok_or(HttpCodegenError::MissingHttpMethod {
                responder: responder.clone(),
            })?;
        let path = arguments
            .string("path")?
            .ok_or(HttpCodegenError::MissingHttpPath {
                responder: responder.clone(),
            })?;

        let mut layers = Vec::new();

        for matched in AttributeQuery::new(item).matched_attributes() {
            if let Some(binding) = bindings
                .iter()
                .find(|binding| matched.matches(&binding.selector))
            {
                layers.push(LayerApplication {
                    marker_value: marker_value(&matched, &responder)?,
                    middleware_field: binding.field.clone(),
                });
            }
        }

        layers.reverse();

        let route_parameters = responder_method(item, &responder, &registries)?;
        let path_parameters =
            path_parameter_names(&path).map_err(|source| HttpCodegenError::InvalidRoutePath {
                responder: responder.clone(),
                path: path.clone(),
                source,
            })?;

        for route_parameter in &route_parameters {
            if !path_parameters.contains(&route_parameter.name) {
                return Err(HttpCodegenError::RouteParameterNotInPath {
                    responder: responder.clone(),
                    parameter: route_parameter.name.clone(),
                    path: path.clone(),
                });
            }
        }

        routes.push(HttpRoute {
            layers,
            method: method
                .segments
                .last()
                .expect("an attribute path has at least one segment")
                .ident
                .clone(),
            path,
            responder_field: format_ident!("{}", item.canonical_path().field_name()),
            responder_path: item.canonical_path().clone(),
            route_parameters,
        });
    }

    Ok(routes)
}
