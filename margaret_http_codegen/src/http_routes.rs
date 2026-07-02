use std::collections::HashMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::build_registry::build_registry;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::interceptor_bindings::interceptor_bindings;
use crate::layer_application::LayerApplication;
use crate::middleware_binding::MiddlewareBinding;
use crate::path_parameter_names::path_parameter_names;
use crate::registries::Registries;
use crate::responder_method::responder_method;
use crate::responder_signature::ResponderSignature;

pub(crate) fn http_routes(
    index: &AttributeIndex,
    bindings: &[MiddlewareBinding],
) -> Result<Vec<HttpRoute>, HttpCodegenError> {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");
    let struct_resolution = index.struct_resolution();
    let binders = build_registry(
        index,
        struct_resolution,
        "provides_route_parameter",
        "Model",
        |binder| HttpCodegenError::HttpRouteParameterBinderModel { binder },
        |model, first, second| HttpCodegenError::AmbiguousHttpRouteParameterBinder {
            model,
            first,
            second,
        },
    )?;
    let trait_resolution = index.trait_resolution();
    let interceptors = interceptor_bindings(index, trait_resolution)?;
    let registries = Registries {
        binders: &binders,
        interceptors: &interceptors,
        struct_resolution,
        trait_resolution,
    };
    let mut routes = Vec::new();
    let mut seen_names: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(HttpCodegenError::RespondsToHttpNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let responder = item.canonical_path().to_string();
        let HttpResponderArguments {
            method,
            name,
            path,
            server,
        } = HttpResponderArguments::parse(&matched.args()?, &responder)?;

        let middleware_selector = AttributeSelector::parse("middleware").expect("a valid selector");
        let mut layers = Vec::new();

        for matched in AttributeQuery::new(item).find_all(&middleware_selector) {
            let arguments = matched.args()?;
            let (Some(tag), None) = (arguments.positional_path(0), arguments.positional(1)) else {
                return Err(HttpCodegenError::MalformedMiddleware {
                    responder: responder.clone(),
                });
            };
            let Some(binding) = bindings
                .iter()
                .find(|binding| binding.selector.matches(tag))
            else {
                return Err(HttpCodegenError::UnknownMiddleware {
                    responder: responder.clone(),
                    tag: tag
                        .segments
                        .last()
                        .expect("a path has at least one segment")
                        .ident
                        .to_string(),
                });
            };

            layers.push(LayerApplication {
                middleware_field: binding.field.clone(),
            });
        }

        layers.reverse();

        let ResponderSignature {
            output: responder_output,
            parameters: route_parameters,
        } = responder_method(item, &responder, &registries)?;
        let path_parameters =
            path_parameter_names(&path).map_err(|source| HttpCodegenError::InvalidRoutePath {
                responder: responder.clone(),
                path: path.clone(),
                source,
            })?;

        for route_parameter in &route_parameters {
            if let Some(path_key) = route_parameter.binding.path_key()
                && !path_parameters.iter().any(|name| name == path_key)
            {
                return Err(HttpCodegenError::RouteParameterNotInPath {
                    responder: responder.clone(),
                    parameter: path_key.to_string(),
                    path: path.clone(),
                });
            }
        }

        if let Some(name) = &name {
            if let Some(first) = seen_names.get(name) {
                return Err(HttpCodegenError::DuplicateRouteName {
                    name: name.clone(),
                    first: first.clone(),
                    second: responder.clone(),
                });
            }

            seen_names.insert(name.clone(), responder.clone());
        }

        routes.push(HttpRoute {
            layers,
            method: method
                .segments
                .last()
                .expect("an attribute path has at least one segment")
                .ident
                .clone(),
            name,
            path,
            responder_field: format_ident!("{}", item.canonical_path().field_name()),
            responder_output,
            responder_path: item.canonical_path().clone(),
            route_parameters,
            server,
        });
    }

    Ok(routes)
}
