use proc_macro2::Ident;
use proc_macro2::Span;

use margaret_attributes::tag::Tag;

use crate::jwks_module_name::JWKS_MODULE_NAME;

#[must_use]
pub fn jwks_endpoint_tag() -> Tag {
    Tag::from_ident(Ident::new(JWKS_MODULE_NAME, Span::call_site()))
}
