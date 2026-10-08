use margaret::framework::accepted_clients::client_keys::ClientKeys;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

use crate::routes::public::get_sign_in_callback::GetSignInCallback;

#[admits_oauth_client(
    cluster_app,
    authentication = ClientAuthenticationMethod::PrivateKeyJwt(
        client_credentials(scopes = ["profile"]),
        introspection,
        keys = ClientKeys::Own,
    ),
    authorization_code(
        consent = ConsentPolicy::Prompted,
        id_token_signing = IdTokenSigning::Rsa,
        redirect_routes = [GetSignInCallback],
        scopes = ["openid", "profile"],
        refresh_token,
    ),
    client_id = "cluster",
    resources = [notes],
)]
pub struct AcceptedClusterClient;
