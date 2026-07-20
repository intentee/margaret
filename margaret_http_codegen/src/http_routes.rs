use std::collections::HashMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::format_path::format_path;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::build_registry::build_registry;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_responder_arguments::HttpResponderArguments;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::layer_application::LayerApplication;
use crate::middleware_plan::MiddlewarePlan;
use crate::responder_method::responder_method;
use crate::responder_selectors::ResponderSelectors;

pub(crate) fn http_routes(
    index: &AttributeIndex,
    middleware_plans: &[MiddlewarePlan],
) -> Result<HttpRouteTable, HttpCodegenError> {
    let selector = AttributeSelector::from_marker("responds_to_http");
    let binders = build_registry(
        index,
        "provides_route_parameter",
        "Model",
        |binder| HttpCodegenError::HttpRouteParameterBinderNotAStruct { binder },
        |binder| HttpCodegenError::HttpRouteParameterBinderModel { binder },
        |model, first, second| HttpCodegenError::AmbiguousHttpRouteParameterBinder {
            model,
            first,
            second,
        },
    )?;
    let middleware_selector = AttributeSelector::from_marker("middleware");
    let responder_selectors = ResponderSelectors::new();
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

        let mut layers = Vec::new();

        for matched in AttributeQuery::new(item).find_all(&middleware_selector) {
            let arguments = matched.args()?;
            let (Some(tag), None) = (arguments.positional_path(0), arguments.positional(1)) else {
                return Err(HttpCodegenError::MalformedMiddleware {
                    responder: responder.clone(),
                });
            };
            let Some(plan) = middleware_plans
                .iter()
                .find(|plan| plan.selector.matches(tag))
            else {
                return Err(HttpCodegenError::UnknownMiddleware {
                    responder: responder.clone(),
                    tag: format_path(tag),
                });
            };

            layers.push(LayerApplication {
                field: plan.field.clone(),
                injects_routes: plan.injects_routes,
                wrapper: plan.wrapper.clone(),
            });
        }

        layers.reverse();

        let arguments = responder_method(
            index,
            item,
            &responder,
            &server,
            &binders,
            &responder_selectors,
        )?;
        let route_path = RoutePath::parse(&path);

        for argument in &arguments {
            if let Some(path_key) = argument.binding.path_key()
                && !route_path.parameters().any(|name| name == path_key)
            {
                return Err(HttpCodegenError::RouteParameterNotInPath {
                    responder: responder.clone(),
                    parameter: path_key.to_string(),
                    path: path.clone(),
                });
            }
        }

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
