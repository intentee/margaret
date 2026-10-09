use margaret::framework::accepted_clients::client_keys::ClientKeys;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;

use crate::auth::profile_scope::ProfileScope;

#[admits_oauth_client(
    partner_app,
    authentication = ClientAuthenticationMethod::PrivateKeyJwt(
        client_credentials(scopes = [ProfileScope]),
        introspection,
        keys = ClientKeys::Published(
            jwks_uri = "https://localhost:20444/partner/jwks.json",
            signing = JwsAlgorithm::Es256,
        ),
    ),
    authorization_code(
        consent = ConsentPolicy::Prompted,
        id_token_signing = IdTokenSigning::EllipticCurve,
        redirect_uris = ["https://localhost:20444/partner/callback"],
        scopes = [OpenidScope, ProfileScope],
        refresh_token,
    ),
    client_id = "partner",
    resources = [notes],
)]
pub struct AcceptedPartnerClient;
