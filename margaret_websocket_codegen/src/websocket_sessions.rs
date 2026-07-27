use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_route_parameter_codegen::route_path::RoutePath;
use quote::format_ident;

use crate::build_for_session_method::build_for_session_method;
use crate::session_arguments::SessionArguments;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_session::WebSocketSession;

pub(crate) fn websocket_sessions(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    middleware_plans: &[MiddlewarePlan],
    registries: &BindingRegistries,
) -> Result<Vec<WebSocketSession>, WebSocketCodegenError> {
    let mut sessions = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::WebsocketSession) {
        let item = matched.item();
        let session = item.canonical_path().to_string();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(WebSocketCodegenError::SessionNotAStruct { session });
        };

        let SessionArguments { path, server } = SessionArguments::parse(matched.args()?, &session)?;
        let method = build_for_session_method(item, &session)?;

        let route_path = RoutePath::parse(&path);
        let subject = format!("session '{session}'");
        let parameters = classify_parameters(
            index,
            item,
            method,
            &BindingContext::Handshake {
                container_bindings: bindings,
                route_path: &route_path,
                server: &server,
                subject: &subject,
            },
            registries,
        )?;
        let layers = resolve_layers(item, middleware_plans, &subject)?;

        if let Some(application) = layers.iter().find(|application| application.injects_views) {
            return Err(WebSocketCodegenError::SessionMiddlewareRendersViews {
                session,
                middleware: application.concrete.to_string(),
            });
        }

        sessions.push(WebSocketSession {
            layers,
            method_name: format_ident!("{}", method.identifier()),
            module_name: identifier.field().to_string(),
            parameters,
            path,
            server,
            session_path: item.canonical_path().clone(),
        });
    }

    sessions.sort_by_key(|session| session.session_path.to_string());

    Ok(sessions)
}
