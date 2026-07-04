use proc_macro2::Ident;
use quote::format_ident;

use crate::provider::Provider;

pub(crate) fn field_ident(provider: &Provider) -> Ident {
    format_ident!("{}", provider.field_name)
}
