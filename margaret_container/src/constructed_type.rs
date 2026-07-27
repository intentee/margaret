use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::provided_type::ProvidedType;
use crate::provider::Provider;
use crate::provides_endpoint_path::provides_endpoint_path;

pub(crate) fn constructed_type(provider: &Provider) -> TokenStream {
    match &provider.provided {
        ProvidedType::Concrete(path) => path_tokens(path),
        ProvidedType::Endpoint(_) => {
            let interface = path_tokens(&provides_endpoint_path());

            quote! { dyn #interface }
        }
        ProvidedType::UriSelected(trait_path) => {
            let interface = path_tokens(trait_path);

            quote! { dyn #interface }
        }
    }
}
