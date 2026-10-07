use margaret::framework::macros::accepts_oauth_client;

#[accepts_oauth_client(
    authentication = private_key_jwt(
        client_credentials(scopes = ["artifacts:read"]),
        introspection,
        jwks_uri = "https://portal.fixture/jwks.json"
    ),
    authorization_code(
        consent = prompted,
        id_token_signing = rsa,
        redirect_uris = ["https://portal.fixture/callback"],
        scopes = ["openid"],
        refresh_token
    ),
    client_id = "portal",
    resources = ["artifacts"],
    token_exchange
)]
pub struct PortalClient;
