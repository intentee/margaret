use syn::ReturnType;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_request_binding_codegen::route_parameter_binders::route_parameter_binders;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::build_for_session_method::build_for_session_method;
use crate::session_arguments::SessionArguments;
use crate::websocket_codegen_error::WebSocketCodegenError;
use crate::websocket_session::WebSocketSession;

fn returns_self(method: &IndexedMethod) -> bool {
    let ReturnType::Type(_, return_type) = &method.signature().output else {
        return false;
    };

    matches!(return_type.as_ref(), Type::Path(type_path) if type_path.path.is_ident("Self"))
}

pub(crate) fn websocket_sessions(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<Vec<WebSocketSession>, WebSocketCodegenError> {
    let selector = AttributeSelector::from_marker("websocket_session");
    let binders = route_parameter_binders(index)?;
    let mut sessions = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let session = item.canonical_path().to_string();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(WebSocketCodegenError::SessionNotAStruct { session });
        };

        let SessionArguments { path, server } = SessionArguments::parse(matched.args()?, &session)?;
        let method = build_for_session_method(item, &session)?;

        if !returns_self(method) {
            return Err(WebSocketCodegenError::BuildForSessionReturnTypeMismatch { session });
        }

        let route_path = RoutePath::parse(&path);
        let subject = format!("session '{session}'");
        let parameters = classify_parameters(
            index,
            item,
            method.signature(),
            &BindingContext::Handshake {
                container_bindings: bindings,
                route_path: &route_path,
                server: &server,
                subject: &subject,
            },
            &binders,
        )?;

        sessions.push(WebSocketSession {
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
