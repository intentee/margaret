use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn string_slice_tokens(values: &[String]) -> TokenStream {
    let items = values.iter().map(|value| Literal::string(value));

    quote! { &[#(#items),*] }
}
