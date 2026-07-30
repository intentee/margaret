#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GeneratedFeature {
    AssetBag,
    AuthenticatedUsers,
    Console,
    Http,
    Jwks,
    Middleware,
    Models,
    Serves,
    Views,
    Websockets,
}
