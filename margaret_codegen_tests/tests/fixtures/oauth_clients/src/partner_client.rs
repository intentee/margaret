use margaret::framework::macros::oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

#[oauth_client(
    partner_client,
    authentication = ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = "PARTNER_CLIENT_SECRET"),
    client_id = "partner",
    issuer = partner,
)]
pub struct PartnerClient;
