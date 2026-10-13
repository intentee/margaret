use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::provider::Provider;

pub(crate) fn field_type(provider: &Provider) -> TokenStream {
    let constructed = path_tokens(&provider.concrete_path);

    quote! { ::std::sync::Arc<#constructed> }
}
