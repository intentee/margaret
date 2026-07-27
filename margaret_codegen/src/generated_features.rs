#[derive(Clone, Copy)]
pub(crate) struct GeneratedFeatures {
    pub(crate) has_asset_bag: bool,
    pub(crate) has_authenticated_users: bool,
    pub(crate) has_console: bool,
    pub(crate) has_http: bool,
    pub(crate) has_jwks: bool,
    pub(crate) has_middleware: bool,
    pub(crate) has_models: bool,
    pub(crate) has_views: bool,
    pub(crate) has_websockets: bool,
    pub(crate) serves: bool,
}
