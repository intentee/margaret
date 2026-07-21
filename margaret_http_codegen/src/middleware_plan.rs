use proc_macro2::Ident;

use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub(crate) struct MiddlewarePlan {
    pub(crate) concrete: CanonicalPath,
    pub(crate) field: Ident,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) selector: AttributeSelector,
    pub(crate) wrapper: Ident,
}

impl MiddlewarePlan {
    pub(crate) fn injects_peer_spiffe_id(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
    }

    pub(crate) fn injects_routes(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Routes))
    }

    pub(crate) fn injects_views(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Views))
    }
}
