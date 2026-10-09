use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;

#[admits_oauth_client(
    spa_app,
    authentication = ClientAuthenticationMethod::None,
    authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = ["http://127.0.0.1:8080/callback"], scopes = [OpenidScope]),
    client_id = "spa",
    resources = [reports],
)]
pub struct SpaClient;
