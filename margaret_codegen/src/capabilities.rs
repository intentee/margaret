use margaret_attributes::attribute_index::AttributeIndex;

#[derive(Clone, Copy)]
pub(crate) struct Capabilities {
    pub(crate) has_asset_bag: bool,
    pub(crate) has_authenticated_users: bool,
    pub(crate) has_console: bool,
    pub(crate) has_http: bool,
    pub(crate) has_jwks: bool,
    pub(crate) has_middleware: bool,
    pub(crate) has_models: bool,
    pub(crate) has_postgres_pool: bool,
    pub(crate) has_views: bool,
    pub(crate) has_websockets: bool,
    pub(crate) serves: bool,
}

impl Capabilities {
    pub(crate) fn detect(index: &AttributeIndex, has_asset_bag: bool) -> Self {
        let has_authenticated_users =
            margaret_request_binding_codegen::has_authenticated_users::has_authenticated_users(
                index,
            );
        let has_http = margaret_http_codegen::has_responders::has_responders(index);
        let has_middleware = margaret_middleware_codegen::has_middleware::has_middleware(index);
        let has_models = margaret_model_codegen::has_models::has_models(index);
        let has_services = margaret_service_codegen::has_services::has_services(index);
        let has_views = margaret_views_codegen::has_views::has_views(index) && has_http;
        let has_websockets =
            margaret_websocket_codegen::has_websocket_sessions::has_websocket_sessions(index);
        let serves = has_http || has_services || has_websockets;
        let has_console =
            margaret_console_codegen::has_commands::has_commands(index) || serves || has_models;

        Self {
            has_asset_bag,
            has_authenticated_users,
            has_console,
            has_http,
            has_jwks: false,
            has_middleware,
            has_models,
            has_postgres_pool: false,
            has_views,
            has_websockets,
            serves,
        }
    }
}
