use margaret::framework::macros::acts_as_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

#[acts_as_oauth_client(
    partner_client,
    authentication = ClientAuthenticationMethod::ClientSecretBasic(client_secret_from = "PARTNER_CLIENT_SECRET"),
    client_id = "partner",
    issuer = partner,
)]
pub struct PartnerClient;
