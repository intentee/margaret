use std::sync::Arc;

use tokio::net::TcpListener;

use crate::bound_server::BoundServer;
use crate::handler::Handler;
use crate::router::Router;

pub struct Server {
    app: Arc<dyn Handler>,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Self {
            app: Arc::new(router),
        }
    }

    pub async fn bind(self, address: &str) -> std::io::Result<BoundServer> {
        let listener = TcpListener::bind(address).await?;

        Ok(BoundServer::new(self.app, listener))
    }
}

#[cfg(test)]
mod tests {
    use super::Server;
    use crate::router::Router;

    #[tokio::test]
    async fn bind_fails_for_an_invalid_address() {
        let server = Server::new(Router::empty());

        assert!(server.bind("this is not an address").await.is_err());
    }
}
