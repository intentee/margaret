use margaret::framework::macros::oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

use crate::profile_scope::ProfileScope;

#[oauth_client(
    partner_client,
    authentication = ClientAuthenticationMethod::PrivateKeyJwt,
    client_id = "partner",
    issuer = partner,
    sign_in(scopes = [ProfileScope]),
)]
pub struct PartnerClient;
