use crate::route_path::RoutePath;

pub(crate) enum AdmittedRoutePath {
    Responder(RoutePath),
    WebSocketSession,
}
