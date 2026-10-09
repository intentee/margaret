use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;

/// # Panics
///
/// Panics when the fixture callback is not a url.
#[must_use]
pub fn fixture_sign_in_flow(
    server: Arc<AuthorizationServerClient>,
    secrets: Arc<JwksSecretHolder>,
) -> Arc<SignInFlow> {
    Arc::new(
        SignInFlow::create(
            server,
            secrets,
            "https://client.example/callback".to_string(),
            &["profile"],
        )
        .expect("the fixture callback is a url"),
    )
}
