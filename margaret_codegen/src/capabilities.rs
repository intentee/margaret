use margaret_attributes::attribute_index::AttributeIndex;

#[derive(Clone, Copy)]
pub(crate) struct Capabilities {
    pub(crate) has_asset_bag: bool,
    pub(crate) has_console: bool,
    pub(crate) has_http: bool,
    pub(crate) has_views: bool,
    pub(crate) serves: bool,
}

impl Capabilities {
    pub(crate) fn detect(index: &AttributeIndex, has_asset_bag: bool) -> Self {
        let has_http = margaret_http_codegen::has_responders::has_responders(index);
        let has_services = margaret_service_codegen::has_services::has_services(index);
        let has_views = margaret_views_codegen::has_views::has_views(index);
        let serves = has_http || has_services;
        let has_console = margaret_console_codegen::has_commands::has_commands(index) || serves;

        Self {
            has_asset_bag,
            has_console,
            has_http,
            has_views,
            serves,
        }
    }
}
