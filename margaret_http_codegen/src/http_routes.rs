use std::num::NonZeroU64;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_injection_codegen::process_method::process_method;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_request_binding_codegen::responder_content::ResponderContent;
use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::application_responder::ApplicationResponder;
use crate::content_reading::ContentReading;
use crate::declared_route::DeclaredRoute;
use crate::declared_routes::DeclaredRoutes;
use crate::framework_input::FrameworkInput;
use crate::framework_responder::FrameworkResponder;
use crate::framework_responders::FrameworkResponders;
use crate::framework_route::FrameworkRoute;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::route_content::RouteContent;
use crate::route_responder::RouteResponder;

fn route_content<TBinding>(
    reading: ContentReading<TBinding>,
    max_body_bytes: Option<NonZeroU64>,
    method: RouteMethod,
    responder: &str,
) -> Result<RouteContent<TBinding>, HttpCodegenError> {
    match reading {
        ContentReading::Unread => match max_body_bytes {
            None => Ok(RouteContent::Unread { method }),
            Some(_) => Err(HttpCodegenError::UnusedBodyLimit {
                responder: responder.to_string(),
            }),
        },
        ContentReading::Read(binding) => match ContentMethod::of(method) {
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

fn application_responder(
    index: &AttributeIndex,
    item: &IndexedItem,
    route: &DeclaredRoute,
    subject: &str,
    registries: &BindingRegistries,
) -> Result<ApplicationResponder, HttpCodegenError> {
    let handler_method = process_method(item)?;
    let arguments = classify_parameters(
        index,
        item,
        handler_method,
        &BindingContext::Responder {
            route_path: &route.path,
            server: &route.server,
            subject,
        },
        registries,
    )?;
    let reading = match ResponderContent::of(&arguments, subject)? {
        ResponderContent::Read(binding) => ContentReading::Read(binding),
        ResponderContent::Unread => ContentReading::Unread,
    };
    let content = route_content(
        reading,
        route.max_body_bytes,
        route.method,
        &item.canonical_path().to_string(),
    )?;

    Ok(ApplicationResponder {
        arguments,
        content,
        is_async: handler_method.signature().asyncness.is_some(),
        method_name: format_ident!("{}", handler_method.identifier()),
        responder_field: format_ident!("{}", route.identifier.field()),
    })
}

fn framework_route(
    FrameworkResponder { handler, input }: FrameworkResponder,
    route: &DeclaredRoute,
) -> Result<FrameworkRoute, HttpCodegenError> {
    let reading = match input {
        FrameworkInput::Content => ContentReading::Read(()),
        FrameworkInput::Head => ContentReading::Unread,
    };

    Ok(FrameworkRoute {
        content: route_content(
            reading,
            route.max_body_bytes,
            route.method,
            &route.item.canonical_path().to_string(),
        )?,
        handler,
    })
}

pub(crate) fn http_routes(
    index: &AttributeIndex,
    routes: DeclaredRoutes,
    FrameworkResponders { mut responders }: FrameworkResponders,
    middleware_plans: &MiddlewarePlans,
    registries: &BindingRegistries,
) -> Result<HttpRouteTable, HttpCodegenError> {
    let mut table = HttpRouteTable::new();

    for route in routes.routes {
        let item = route.item;
        let responder_path = item.canonical_path().clone();
        let subject = format!("responder '{responder_path}'");
        let layers = resolve_layers(item, middleware_plans, &subject)?;
        let responder = match responders.remove(&responder_path) {
            Some(framework) => RouteResponder::Framework(framework_route(framework, &route)?),
            None => RouteResponder::Application(application_responder(
                index, item, &route, &subject, registries,
            )?),
        };
        let DeclaredRoute {
            name, path, server, ..
        } = route;

        table.insert(
            path,
            HttpRoute {
                layers,
                name,
                responder,
                responder_path,
                server,
            },
        )?;
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use margaret_route_method::route_method::RouteMethod;

    use super::route_content;
    use crate::content_reading::ContentReading;
    use crate::http_codegen_error::HttpCodegenError;
    use crate::route_content::RouteContent;

    #[test]
    fn reads_no_body_of_a_route_that_does_not_read_one() {
        assert!(matches!(
            route_content::<()>(
                ContentReading::Unread,
                None,
                RouteMethod::Get,
                "crate::Route"
            ),
            Ok(RouteContent::Unread { method }) if method == RouteMethod::Get
        ));
    }

    #[test]
    fn keeps_the_limit_of_a_route_that_reads_its_body() {
        assert!(matches!(
            route_content(
                ContentReading::Read(()),
                std::num::NonZeroU64::new(5),
                RouteMethod::Post,
                "crate::Route",
            ),
            Ok(RouteContent::Read { limit, .. }) if limit.get() == 5
        ));
    }

    #[test]
    fn rejects_a_body_read_by_a_get_route() {
        assert!(matches!(
            route_content(
                ContentReading::Read(()),
                std::num::NonZeroU64::new(5),
                RouteMethod::Get,
                "crate::Route"
            ),
            Err(HttpCodegenError::ContentOnGetRoute { responder }) if responder == "crate::Route"
        ));
    }
}
