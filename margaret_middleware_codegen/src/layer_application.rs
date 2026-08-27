use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_request_binding_codegen::request_body_intake::RequestBodyIntake;

pub struct LayerApplication {
    pub body_intake: RequestBodyIntake,
    pub concrete: CanonicalPath,
    pub field: Ident,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub wrapper: Ident,
}
