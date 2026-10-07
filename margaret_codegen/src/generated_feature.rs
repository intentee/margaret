#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GeneratedFeature {
    AcceptedClients,
    AssetBag,
    AuthenticatedUsers,
    Console,
    Http,
    Jwks,
    Middleware,
    OAuthClients,
    OidcProvider,
    Schema,
    Serves,
    TokenIssuance,
    TrustedIssuers,
    Views,
    Websockets,
}
