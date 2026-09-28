use std::sync::Arc;

use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;

use crate::AuthVerifier;
use crate::ServerStore;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(
        #[jwks_secret_store(server)] store: Arc<ServerStore>,
        #[jwks_secret_store(client = auth)] verifier: Arc<AuthVerifier>,
    ) -> anyhow::Result<Self> {}
}

#[provides_jwks_endpoint(auth)]
#[singleton]
struct AuthEndpoint;

impl ProvidesEndpoint for AuthEndpoint {}
