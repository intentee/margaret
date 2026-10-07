use margaret::framework::macros::accepts_oauth_client;

#[accepts_oauth_client(
    authentication = none,
    authorization_code(
        consent = implicit,
        id_token_signing = elliptic_curve,
        redirect_uris = ["http://127.0.0.1:8080/callback"],
        scopes = ["openid"]
    ),
    client_id = "spa",
    resources = ["reports"]
)]
pub struct SpaClient;
