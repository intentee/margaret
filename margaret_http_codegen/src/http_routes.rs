use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::matched_attribute::MatchedAttribute;

use crate::build_registry::build_registry;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::interceptor_bindings::interceptor_bindings;
use crate::layer_application::LayerApplication;
use crate::middleware_binding::MiddlewareBinding;
use crate::path_parameter_names::path_parameter_names;
use crate::registries::Registries;
use crate::responder_method::responder_method;
use crate::responder_signature::ResponderSignature;
use crate::route_parameter_binding::RouteParameterBinding;

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

fn is_path_parameter(binding: &RouteParameterBinding) -> bool {
    matches!(
        binding,
        RouteParameterBinding::Raw | RouteParameterBinding::Bound { .. }
    )
}

fn needs_authenticated_actor(route_parameters: &[crate::route_parameter::RouteParameter]) -> bool {
    route_parameters.iter().any(|route_parameter| {
        matches!(
            &route_parameter.binding,
            RouteParameterBinding::SessionAuthenticated(_)
                | RouteParameterBinding::Bound {
                    intent: Some(_),
                    ..
                }
        )
    })
}

fn site_action_guards(
    item: &margaret_attributes::indexed_item::IndexedItem,
    responder: &str,
    site_gates: &HashMap<Path, String>,
) -> Result<Vec<Path>, HttpCodegenError> {
    let can_selector = AttributeSelector::parse("can").expect("a valid selector");
    let mut guards = Vec::new();

    for matched in AttributeQuery::new(item).matched_attributes() {
        if !matched.matches(&can_selector) {
            continue;
        }

        let action = matched
            .args()?
            .positional_path(0)
            .ok_or_else(|| HttpCodegenError::CanWithoutAction {
                responder: responder.to_string(),
            })?
            .clone();

        if !site_gates.contains_key(&action) {
            return Err(HttpCodegenError::MissingSiteActionGate {
                responder: responder.to_string(),
                action: quote! { #action }.to_string(),
            });
        }

        guards.push(action);
    }

    Ok(guards)
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    bindings: &[MiddlewareBinding],
    site_gates: &HashMap<Path, String>,
    has_store: bool,
) -> Result<Vec<HttpRoute>, HttpCodegenError> {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");
    let struct_paths = index.struct_paths();
    let binders = build_registry(
        index,
        &struct_paths,
        "provides_route_parameter",
        "Model",
        |binder| HttpCodegenError::HttpRouteParameterBinderModel { binder },
        |model, first, second| HttpCodegenError::AmbiguousHttpRouteParameterBinder {
            model,
            first,
            second,
        },
    )?;
    let gates = build_registry(
        index,
        &struct_paths,
        "decides_crud_action",
        "Subject",
        |gate| HttpCodegenError::CrudGateSubject { gate },
        |subject, first, second| HttpCodegenError::AmbiguousCrudGate {
            subject,
            first,
            second,
        },
    )?;
    let trait_paths = index.trait_paths();
    let interceptors = interceptor_bindings(index, &trait_paths)?;
    let registries = Registries {
        binders: &binders,
        gates: &gates,
        interceptors: &interceptors,
        struct_paths: &struct_paths,
        trait_paths: &trait_paths,
    };
    let mut routes = Vec::new();
    let mut seen_symbols: HashMap<String, String> = HashMap::new();

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

        let ResponderSignature {
            output: responder_output,
            parameters: route_parameters,
        } = responder_method(item, &responder, &registries)?;
        let guards = site_action_guards(item, &responder, site_gates)?;
        let path_parameters =
            path_parameter_names(&path).map_err(|source| HttpCodegenError::InvalidRoutePath {
                responder: responder.clone(),
                path: path.clone(),
                source,
            })?;

        for route_parameter in &route_parameters {
            if is_path_parameter(&route_parameter.binding)
                && !path_parameters.contains(&route_parameter.name)
            {
                return Err(HttpCodegenError::RouteParameterNotInPath {
                    responder: responder.clone(),
                    parameter: route_parameter.name.clone(),
                    path: path.clone(),
                });
            }
        }

        if (!guards.is_empty() || needs_authenticated_actor(&route_parameters)) && !has_store {
            return Err(HttpCodegenError::NoAuthenticatedActorStore {
                responder: responder.clone(),
            });
        }

        let symbol_name = item
            .canonical_path()
            .segments()
            .last()
            .expect("a canonical path has at least one segment")
            .clone();

        if let Some(first) = seen_symbols.get(&symbol_name) {
            return Err(HttpCodegenError::DuplicateRouteSymbol {
                symbol: symbol_name,
                first: first.clone(),
                second: responder.clone(),
            });
        }

        seen_symbols.insert(symbol_name.clone(), responder.clone());

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
            responder_output,
            responder_path: item.canonical_path().clone(),
            route_parameters,
            route_symbol_key: responder.clone(),
            route_symbol_variant: format_ident!("{}", symbol_name),
            site_action_guards: guards,
        });
    }

    Ok(routes)
}
