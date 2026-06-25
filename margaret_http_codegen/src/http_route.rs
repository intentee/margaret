use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::matched_attribute::MatchedAttribute;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::http_codegen_error::HttpCodegenError;
use crate::middleware_binding::MiddlewareBinding;
use crate::path_parameter_names::path_parameter_names;
use crate::responder_method::responder_method;
use crate::route_parameter::RouteParameter;

pub(crate) struct LayerApplication {
    pub(crate) marker_value: TokenStream,
    pub(crate) middleware_field: Ident,
    pub(crate) priority: i64,
}

pub(crate) struct HttpRoute {
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: Ident,
    pub(crate) path: String,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) route_parameters: Vec<RouteParameter>,
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    bindings: &[MiddlewareBinding],
) -> Result<Vec<HttpRoute>, HttpCodegenError> {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");
    let mut routes = Vec::new();

    for matched in index.select(&selector) {
        let item = match matched.holder() {
            AttributeHolder::Item(item) if item.kind().is_struct() => item,
            holder => {
                return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                    target: holder.target_path(),
                });
            }
        };

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

        for binding in bindings {
            for marker in AttributeQuery::new(matched.holder()).find_all(&binding.selector) {
                layers.push(LayerApplication {
                    marker_value: marker_value(&marker, &responder)?,
                    middleware_field: binding.field.clone(),
                    priority: binding.priority,
                });
            }
        }

        layers.sort_by_key(|application| application.priority);

        let route_parameters = responder_method(index, item.canonical_path(), &responder)?;
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
