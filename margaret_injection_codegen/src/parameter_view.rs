use proc_macro2::Ident;
use syn::Attribute;
use syn::Type;

pub struct ParameterView<'signature> {
    pub attributes: &'signature [Attribute],
    pub declared: &'signature Type,
    pub holder: Ident,
    pub position: usize,
}
