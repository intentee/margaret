use margaret::framework::macros::oauth_client;

#[oauth_client(
    blog,
    authentication = private_key_jwt,
    client_id = "blog",
    issuer = margaret
)]
pub struct BlogClient;
