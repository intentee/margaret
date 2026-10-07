use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::portal_callback::PORTAL_CALLBACK;

pub const PORTAL_CLIENT: AcceptedClient = AcceptedClient {
    authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
        consent: ConsentPolicy::Implicit,
        id_token_signing: IdTokenSigning::Rsa,
        redirect_uris: &[PORTAL_CALLBACK],
        refresh: RefreshTokenGrant::Granted,
        scopes: &["openid", "profile"],
    }),
    client_id: "portal",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Granted,
};
