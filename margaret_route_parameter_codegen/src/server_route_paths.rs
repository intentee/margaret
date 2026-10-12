use std::collections::BTreeMap;

use matchit::InsertError;
use matchit::Router;

use crate::admitted_route_path::AdmittedRoutePath;
use crate::route_path::RoutePath;
use crate::route_path_error::RoutePathError;

#[derive(Default)]
pub(crate) struct ServerRoutePaths {
    admitted: BTreeMap<String, AdmittedRoutePath>,
    matcher: Router<()>,
}

impl ServerRoutePaths {
    pub(crate) fn admit_responder_path(
        &mut self,
        pattern: &str,
    ) -> Result<RoutePath, RoutePathError> {
        match self.admitted.get(pattern) {
            Some(AdmittedRoutePath::Responder(path)) => Ok(path.clone()),
            Some(AdmittedRoutePath::WebSocketSession) => {
                Err(RoutePathError::ServedByWebSocketSession)
            }
            None => {
                let path = self.routed(pattern)?;

                self.admitted.insert(
                    pattern.to_owned(),
                    AdmittedRoutePath::Responder(path.clone()),
                );

                Ok(path)
            }
        }
    }

    pub(crate) fn admit_web_socket_session_path(
        &mut self,
        pattern: &str,
    ) -> Result<RoutePath, RoutePathError> {
        match self.admitted.get(pattern) {
            Some(AdmittedRoutePath::Responder(_)) => Err(RoutePathError::ServedByResponder),
            Some(AdmittedRoutePath::WebSocketSession) | None => {
                let path = self.routed(pattern)?;

                self.admitted
                    .insert(pattern.to_owned(), AdmittedRoutePath::WebSocketSession);

                Ok(path)
            }
        }
    }

    fn routed(&mut self, pattern: &str) -> Result<RoutePath, RoutePathError> {
        self.matcher
            .insert(pattern, ())
            .map_err(|source| match source {
                InsertError::Conflict { with } => RoutePathError::ConflictingRoutePath {
                    conflicting_path: with,
                },
                source => RoutePathError::RejectedByRouter { source },
            })?;

        RoutePath::admitted(pattern)
    }
}
