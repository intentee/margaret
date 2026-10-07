use margaret::framework::macros::accepts_oauth_client;

#[accepts_oauth_client(
    authentication = private_key_jwt(
        client_credentials(scopes = ["profile"]),
        introspection,
        jwks_uri = "https://issuer.internal/.well-known/jwks.json"
    ),
    authorization_code(
        consent = prompted,
        id_token_signing = rsa,
        redirect_uris = ["https://public.localhost/sign-in/callback"],
        scopes = ["openid", "profile"]
    ),
    client_id = "blog",
    resources = ["attachments"]
)]
pub struct AcceptedBlogClient;
