use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn construction_error_path() -> TokenStream {
    quote! { margaret::framework::container_error::construction_error::ConstructionError }
}
