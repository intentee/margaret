use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

#[derive(Debug, Eq, PartialEq)]
pub struct ConsentPage {
    pub decision: RouteUrlInput,
    pub view: CanonicalPath,
}
