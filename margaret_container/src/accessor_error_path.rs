use proc_macro2::TokenStream;
use quote::quote;

use crate::construction_error_path::construction_error_path;

#[must_use]
pub fn accessor_error_path() -> TokenStream {
    let error = construction_error_path();

    quote! { std::sync::Arc<#error> }
}
