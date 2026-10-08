use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_model_codegen::resolved_rust_type::ResolvedRustType;

pub(crate) fn resolved_type_tokens(resolved: &ResolvedRustType) -> TokenStream {
    let path = path_tokens(&resolved.path);

    if resolved.arguments.is_empty() {
        path
    } else {
        let arguments = resolved.arguments.iter().map(resolved_type_tokens);

        quote! { #path<#(#arguments),*> }
    }
}
