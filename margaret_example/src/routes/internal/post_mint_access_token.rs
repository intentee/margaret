use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::jwks::MintAccessTokenHandler;
use crate::system_clock::SystemClock;

#[singleton]
#[responds_to_http(method = "post", path = "/.well-known/mint", server = "internal")]
pub struct PostMintAccessToken {
    clock: Arc<SystemClock>,
    mint_access_token_handler: Arc<MintAccessTokenHandler>,
}

impl PostMintAccessToken {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        mint_access_token_handler: Arc<MintAccessTokenHandler>,
    ) -> anyhow::Result<Self> {
        Ok({
            Self {
                clock,
                mint_access_token_handler,
            }
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self, request: &Request) -> anyhow::Result<Response> {
        Ok({
            self.mint_access_token_handler
                .respond(request, self.clock.now())
                .await
        })
    }
}
