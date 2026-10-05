use std::collections::BTreeSet;

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

/// # Panics
///
/// Panics when the fixture client identifier, callback, resource or scope is rejected.
#[must_use]
pub fn fixture_client(
    client_id: &str,
    authentication: AcceptedClientAuthentication,
) -> AcceptedClient {
    AcceptedClient {
        authentication,
        authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
            consent: ConsentPolicy::Prompted,
            id_token_signing: IdTokenSigning::Rsa,
            redirect_uris: NonEmptySet::of(
                Url::parse("https://client.example/callback").expect("the callback is a url"),
                [],
            ),
            refresh: RefreshTokenGrant::Granted,
            scopes: BTreeSet::from(["openid".parse().expect("the scope is a scope token")]),
        }),
        client_id: client_id.parse().expect("the client identifier is visible"),
        resources: NonEmptySet::of(
            "artifacts".parse().expect("the resource is an audience"),
            [],
        ),
        token_exchange: TokenExchangeGrant::Withheld,
    }
}
