use proc_macro2::Ident;

use margaret_attributes::attribute_selector::AttributeSelector;

pub(crate) struct MiddlewareBinding {
    pub(crate) field: Ident,
    pub(crate) selector: AttributeSelector,
}
