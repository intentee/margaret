use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::provided_type::ProvidedType;

pub(crate) fn constructed_type(provided: &ProvidedType) -> TokenStream {
    match provided {
        ProvidedType::Concrete(path) => path_tokens(path),
        ProvidedType::UriSelected(trait_path) => {
            let interface = path_tokens(trait_path);

            quote! { dyn #interface }
        }
    }
}
