use std::sync::Arc;

use crate::handler_name::HandlerName;
use crate::matchit::InsertError;
use crate::method_handler::MethodHandler;
use crate::named_handler::NamedHandler;
use crate::route_entry::RouteEntry;
use crate::router::Router;

fn named_handlers(entries: &[RouteEntry]) -> Vec<NamedHandler> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            RouteEntry::Http { handlers, .. } => Some(handlers),
            RouteEntry::WebSocket { .. } => None,
        })
        .flatten()
        .filter_map(|MethodHandler { handler, name, .. }| match name {
            HandlerName::Anonymous => None,
            HandlerName::Named(name) => Some(NamedHandler::new(name, Arc::clone(handler))),
        })
        .collect()
}

pub struct ServerRoutes {
    pub named_handlers: Vec<NamedHandler>,
    pub router: Router,
}

impl ServerRoutes {
    /// # Errors
    ///
    /// Returns `InsertError` when two entries claim the same path.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, InsertError> {
        let named_handlers = named_handlers(&entries);

        Ok(Self {
            named_handlers,
            router: Router::build(entries)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use hyper::Method;

    use super::ServerRoutes;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::handler_error::HandlerError;
    use crate::method_handler::MethodHandler;
    use crate::request::Request;
    use crate::resolve_continuation::resolve_continuation;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::route_entry::RouteEntry;

    struct Missing;

    #[async_trait]
    impl Handler for Missing {
        async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
            Ok(ResponseContinuation::Done(Response::not_found()))
        }
    }

    fn not_found() -> Arc<dyn Handler> {
        Arc::new(Missing)
    }

    async fn status_of(continuation: ResponseContinuation) -> u16 {
        resolve_continuation(
            &Arc::new(ForwardTargets::new(Vec::new())),
            Request::new(Method::GET, "/articles".to_string()),
            continuation,
        )
        .await
        .expect("the continuation resolves")
        .into_http()
        .status()
        .as_u16()
    }

    #[test]
    fn names_only_the_handlers_that_declare_a_name() {
        let server_routes = ServerRoutes::build(Vec::from([RouteEntry::new(
            "/articles",
            Vec::from([
                MethodHandler::named("GET", "get_articles", not_found()),
                MethodHandler::anonymous("POST", not_found()),
            ]),
        )]))
        .expect("the entries build a router");

        assert_eq!(server_routes.named_handlers.len(), 1);
    }

    #[tokio::test]
    async fn names_the_very_handler_the_entry_declared() {
        let server_routes = ServerRoutes::build(Vec::from([RouteEntry::new(
            "/articles",
            Vec::from([MethodHandler::named("GET", "get_articles", not_found())]),
        )]))
        .expect("the entries build a router");
        let named = server_routes
            .named_handlers
            .into_iter()
            .next()
            .expect("the named handler is collected");
        let request = Request::new(Method::GET, "/articles".to_string());
        let continuation = named
            .into_handler()
            .handle(&request)
            .await
            .expect("the named handler responds");

        assert_eq!(status_of(continuation).await, 404);
    }

    #[test]
    fn reports_two_entries_claiming_the_same_path() {
        assert!(
            ServerRoutes::build(Vec::from([
                RouteEntry::new(
                    "/items/{id}",
                    Vec::from([MethodHandler::anonymous("GET", not_found())]),
                ),
                RouteEntry::new(
                    "/items/{name}",
                    Vec::from([MethodHandler::anonymous("GET", not_found())]),
                ),
            ]))
            .is_err()
        );
    }

    #[test]
    fn carries_its_router_when_no_handler_is_named() {
        let server_routes =
            ServerRoutes::build(Vec::new()).expect("an empty entry list builds a router");

        assert!(server_routes.named_handlers.is_empty());
    }
}
