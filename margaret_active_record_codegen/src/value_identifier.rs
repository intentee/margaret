use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn value_identifier(position: usize) -> Ident {
    format_ident!("value_{position}")
}
