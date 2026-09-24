use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;

use crate::middleware_injections::MiddlewareInjections;

pub struct MiddlewarePlan {
    pub concrete: CanonicalPath,
    pub(crate) field: Ident,
    pub(crate) injections: MiddlewareInjections,
    pub(crate) is_async: bool,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) tag: Tag,
    pub(crate) wrapper: Ident,
}
