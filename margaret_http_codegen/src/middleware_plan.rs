use proc_macro2::Ident;

use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;

use crate::middleware_argument::MiddlewareArgument;

pub(crate) struct MiddlewarePlan {
    pub(crate) arguments: Vec<MiddlewareArgument>,
    pub(crate) concrete: CanonicalPath,
    pub(crate) field: Ident,
    pub(crate) injects_routes: bool,
    pub(crate) selector: AttributeSelector,
    pub(crate) wrapper: Ident,
}
