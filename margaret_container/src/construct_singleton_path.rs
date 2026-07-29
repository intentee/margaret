use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn construct_singleton_path() -> TokenStream {
    quote! { margaret::framework::construct_singleton::construct_singleton }
}
