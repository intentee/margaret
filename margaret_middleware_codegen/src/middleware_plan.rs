use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub struct MiddlewarePlan {
    pub concrete: CanonicalPath,
    pub(crate) field: Ident,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) tag: Tag,
    pub(crate) wrapper: Ident,
}

impl MiddlewarePlan {
    #[must_use]
    pub fn injects_peer_spiffe_id(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
    }

    #[must_use]
    pub fn injects_routes(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Routes))
    }

    #[must_use]
    pub fn injects_views(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Views))
    }
}
