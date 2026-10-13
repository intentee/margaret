use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_server_origin::route_url::RouteUrl;

/// # Panics
///
/// Panics when the fixture callback is not a url.
#[must_use]
pub fn fixture_sign_in_flow(
    server: Arc<AuthorizationServerClient>,
    secrets: Arc<JwksSecretHolder>,
) -> Arc<SignInFlow> {
    Arc::new(SignInFlow::create(
        server,
        secrets,
        RouteUrl::parse("https://client.example/callback".to_string())
            .expect("the fixture callback is a url"),
        &["profile"],
    ))
}
