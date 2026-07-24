use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::handshake::derive_accept_key;
use tokio_tungstenite::tungstenite::protocol::Role;
use tokio_util::sync::CancellationToken;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::web_socket_upgrade::WebSocketUpgrade;

use crate::serve_web_socket_connection::serve_web_socket_connection;
use crate::web_socket_dispatch_table::WebSocketDispatchTable;
use crate::web_socket_session_factory::WebSocketSessionFactory;

async fn drive_web_socket_upgrade<Session>(
    on_upgrade: OnUpgrade,
    cancellation_token: CancellationToken,
    session: Arc<Session>,
    dispatch_table: Arc<WebSocketDispatchTable<Session>>,
) where
    Session: Send + Sync + 'static,
{
    let Ok(upgraded) = on_upgrade.await else {
        return;
    };
    let stream = WebSocketStream::from_raw_socket(TokioIo::new(upgraded), Role::Server, None).await;

    serve_web_socket_connection(cancellation_token, session, dispatch_table, stream).await;
}

pub struct WebSocketUpgradeEntry<Factory: WebSocketSessionFactory> {
    dispatch_table: Arc<WebSocketDispatchTable<Factory::Session>>,
    factory: Factory,
}

impl<Factory: WebSocketSessionFactory> WebSocketUpgradeEntry<Factory> {
    #[must_use]
    pub fn new(
        factory: Factory,
        dispatch_table: Arc<WebSocketDispatchTable<Factory::Session>>,
    ) -> Self {
        Self {
            dispatch_table,
            factory,
        }
    }
}

#[async_trait]
impl<Factory> WebSocketUpgrade for WebSocketUpgradeEntry<Factory>
where
    Factory: WebSocketSessionFactory + 'static,
    Factory::Session: Send + Sync + 'static,
{
    async fn upgrade(
        self: Arc<Self>,
        handshake: &Request,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
    ) -> ResponseContinuation {
        let Ok(Some(key)) = handshake.inputs.server.header("sec-websocket-key") else {
            return ResponseContinuation::from(Response::text(
                400,
                "the websocket handshake is missing the Sec-WebSocket-Key header",
            ));
        };

        match handshake.inputs.server.header("sec-websocket-version") {
            Ok(Some("13")) => {}
            _ => {
                return ResponseContinuation::from(Response::text(
                    400,
                    "the websocket handshake must request Sec-WebSocket-Version 13",
                ));
            }
        }

        let accept = derive_accept_key(key.as_bytes());
        let session = match self.factory.create(handshake).await {
            Ok(session) => session,
            Err(continuation) => return continuation,
        };

        tokio::spawn(drive_web_socket_upgrade(
            on_upgrade,
            cancellation_token,
            session,
            self.dispatch_table.clone(),
        ));

        ResponseContinuation::from(
            Response::text(101, "")
                .header("connection", "Upgrade")
                .header("sec-websocket-accept", accept)
                .header("upgrade", "websocket"),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use tokio_util::sync::CancellationToken;

    use super::drive_web_socket_upgrade;
    use crate::web_socket_dispatch_table::WebSocketDispatchTable;

    #[tokio::test]
    async fn returns_without_serving_when_the_upgrade_never_completes() {
        let mut request = hyper::Request::new(());
        let on_upgrade = hyper::upgrade::on(&mut request);
        let session = Arc::new(());
        let session_handle = Arc::clone(&session);
        let dispatch_table: Arc<WebSocketDispatchTable<()>> =
            Arc::new(WebSocketDispatchTable::new(HashMap::new(), HashMap::new()));

        drive_web_socket_upgrade(
            on_upgrade,
            CancellationToken::new(),
            session,
            dispatch_table,
        )
        .await;

        assert_eq!(Arc::strong_count(&session_handle), 1);
    }
}
