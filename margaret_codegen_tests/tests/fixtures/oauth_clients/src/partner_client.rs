use margaret::framework::macros::oauth_client;

#[oauth_client(
    partner_client,
    authentication = client_secret_basic(client_secret_from = "PARTNER_CLIENT_SECRET"),
    client_id = "partner",
    issuer = partner
)]
pub struct PartnerClient;
