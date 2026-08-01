use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::provided_type::ProvidedType;
use crate::provider::Provider;

pub(crate) fn constructed_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) | ProvidedType::Endpoint(path) => path_tokens(path),
        ProvidedType::UriSelected(trait_path) => {
            let interface = path_tokens(trait_path);

            quote! { dyn #interface }
        }
    }
}
