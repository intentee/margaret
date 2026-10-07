use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::spa_callback::SPA_CALLBACK;

pub const SPA_CLIENT: AcceptedClient = AcceptedClient {
    authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
        consent: ConsentPolicy::Prompted,
        id_token_signing: IdTokenSigning::EllipticCurve,
        redirect_uris: &[SPA_CALLBACK],
        refresh: RefreshTokenGrant::Withheld,
        scopes: &["openid"],
    }),
    client_id: "spa",
    resources: &["artifacts", "reports"],
    token_exchange: TokenExchangeGrant::Withheld,
};
