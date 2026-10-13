use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

pub(crate) struct MarkedCallback {
    pub(crate) client: Tag,
    pub(crate) landing: RouteUrlInput,
    pub(crate) route: CanonicalPath,
}
