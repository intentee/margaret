use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_scope::ForwardScope;
use margaret_http::server::Server;
use margaret_http::server_route_entries::ServerRouteEntries;

use crate::server_listener::ServerListener;

pub struct ServerService<TTarget: ForwardScope> {
    listener: ServerListener,
    routes: ServerRouteEntries<TTarget>,
}

impl<TTarget: ForwardScope> ServerService<TTarget> {
    #[must_use]
    pub fn new(listener: ServerListener, routes: ServerRouteEntries<TTarget>) -> Self {
        Self { listener, routes }
    }
}

#[async_trait]
impl<TTarget: ForwardScope> Service for ServerService<TTarget> {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self {
            listener:
                ServerListener {
                    address,
                    transport,
                    uploads,
                },
            routes,
        } = *self;
        let bound = BoundServer::bind(
            Arc::new(Server::new(address, transport, uploads)),
            routes.build()?,
        )
        .await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
