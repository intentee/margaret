use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::is_singleton::is_singleton;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_middleware_codegen::resolve_layers::resolve_layers;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_route_parameter_codegen::route_path::RoutePath;
use quote::format_ident;

use crate::access_policy_binding::AccessPolicyBinding;
use crate::build_for_session_method::build_for_session_method;
use crate::session_arguments::SessionArguments;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_session::WebSocketSession;

fn public_access_path() -> margaret_attributes::canonical_path::CanonicalPath {
    margaret_attributes::canonical_path::CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "http".to_string(),
        "public_access".to_string(),
        "PublicAccess".to_string(),
    ])
}

fn written_path(path: &syn::Path) -> margaret_attributes::canonical_path::CanonicalPath {
    margaret_attributes::canonical_path::CanonicalPath::new(
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect(),
    )
}

fn access_policy(
    index: &AttributeIndex,
    item: &margaret_attributes::indexed_item::IndexedItem,
    written: &syn::Path,
    session: &str,
) -> Result<AccessPolicyBinding, WebSocketCodegenError> {
    if written_path(written) == public_access_path() {
        return Ok(AccessPolicyBinding::Public);
    }

    let (path, policy_item) = index
        .resolve_item_path(item, written)
        .and_then(|path| index.item(&path).map(|policy_item| (path, policy_item)))
        .ok_or_else(|| WebSocketCodegenError::UnresolvedSessionAccessPolicy {
            session: session.to_string(),
        })?;
    let Some(identifier) = index.struct_identifier(&path) else {
        return Err(WebSocketCodegenError::SessionAccessPolicyNotAStruct {
            policy: path.to_string(),
            session: session.to_string(),
        });
    };

    if !is_singleton(policy_item) {
        return Err(WebSocketCodegenError::SessionAccessPolicyNotSingleton {
            policy: path.to_string(),
            session: session.to_string(),
        });
    }

    Ok(AccessPolicyBinding::Singleton {
        field: format_ident!("{}", identifier.field()),
        path,
    })
}

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

        let SessionArguments {
            access,
            origin,
            path,
            server,
        } = SessionArguments::parse(matched.args()?, &session)?;
        let access_policy = access_policy(index, item, &access, &session)?;
        let method = build_for_session_method(item, &session)?;

        let route_path = RoutePath::parse(&path).map_err(|source| {
            WebSocketCodegenError::InsecureSessionPath {
                path: path.clone(),
                session: session.clone(),
                source,
            }
        })?;
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
            access_policy,
            layers,
            method_name: format_ident!("{}", method.identifier()),
            module_name: identifier.field().to_string(),
            origin,
            parameters,
            path,
            server,
            session_path: item.canonical_path().clone(),
        });
    }

    sessions.sort_by_key(|session| session.session_path.to_string());

    Ok(sessions)
}
