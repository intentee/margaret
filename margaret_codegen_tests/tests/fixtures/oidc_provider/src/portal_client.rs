use margaret::framework::accepted_clients::client_keys::ClientKeys;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;

use crate::artifacts_read_scope::ArtifactsReadScope;

#[admits_oauth_client(
    portal_app,
    authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(scopes = [ArtifactsReadScope]), introspection, keys = ClientKeys::Published(jwks_uri = "https://portal.fixture/jwks.json", signing = JwsAlgorithm::Es256)),
    authorization_code(consent = ConsentPolicy::Prompted, id_token_signing = IdTokenSigning::Rsa, redirect_uris = ["https://portal.fixture/callback"], scopes = [OpenidScope], refresh_token),
    client_id = "portal",
    resources = [artifacts],
    token_exchange(scopes = []),
)]
pub struct PortalClient;
