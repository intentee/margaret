use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::injects_routes::injects_routes;
use margaret_request_binding_codegen::injects_views::injects_views;
use margaret_request_binding_codegen::request_binding::RequestBinding;
use margaret_request_binding_codegen::request_body_intake::RequestBodyIntake;

pub struct MiddlewarePlan {
    pub concrete: CanonicalPath,
    pub(crate) field: Ident,
    pub(crate) is_async: bool,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) tag: Tag,
    pub(crate) wrapper: Ident,
}

impl MiddlewarePlan {
    #[must_use]
    pub fn body_intake(&self) -> RequestBodyIntake {
        RequestBodyIntake::of(&self.parameters)
    }

    #[must_use]
    pub fn injects_peer_spiffe_id(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
    }

    #[must_use]
    pub fn injects_routes(&self) -> bool {
        injects_routes(&self.parameters)
    }

    #[must_use]
    pub fn injects_views(&self) -> bool {
        injects_views(&self.parameters)
    }
}
