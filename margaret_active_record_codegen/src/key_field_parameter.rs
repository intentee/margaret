use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn key_field_parameter(position: usize) -> Ident {
    format_ident!("Field{position}")
}
