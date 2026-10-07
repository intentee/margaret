use margaret::framework::macros::accepts_oauth_client;

#[accepts_oauth_client(
    authentication = none,
    client_id = "ci",
    resources = ["attachments"],
    token_exchange
)]
pub struct AcceptedCiClient;
