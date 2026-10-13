use proc_macro2::Ident;

use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::content_binding::ContentBinding;

use crate::route_content::RouteContent;

pub(crate) struct ApplicationResponder {
    pub(crate) arguments: Vec<BoundParameter>,
    pub(crate) content: RouteContent<ContentBinding>,
    pub(crate) is_async: bool,
    pub(crate) method_name: Ident,
    pub(crate) responder_field: Ident,
}
