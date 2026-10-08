use proc_macro2::Ident;
use syn::Type;

use margaret_attributes::indexed_attribute::IndexedAttribute;

pub struct ParameterView<'signature> {
    pub attributes: &'signature [IndexedAttribute],
    pub declared: &'signature Type,
    pub name: &'signature Ident,
    pub position: usize,
}
