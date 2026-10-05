use url::Url;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::non_empty_set::NonEmptySet;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::fixture_resources::fixture_resources;
use crate::fixture_scopes::fixture_scopes;
use crate::spa_callback::SPA_CALLBACK;

/// # Panics
///
/// Panics when the fixture client identifier or callback is rejected.
#[must_use]
pub fn spa_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::Public,
        authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
            consent: ConsentPolicy::Prompted,
            id_token_signing: IdTokenSigning::EllipticCurve,
            redirect_uris: NonEmptySet::of(
                Url::parse(SPA_CALLBACK).expect("the callback is a url"),
                [],
            ),
            refresh: RefreshTokenGrant::Withheld,
            scopes: fixture_scopes(&["openid"]),
        }),
        client_id: "spa".parse().expect("the spa identifier is visible"),
        resources: fixture_resources("artifacts", &["reports"]),
        token_exchange: TokenExchangeGrant::Withheld,
    }
}
