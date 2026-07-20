use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::protocol::Protocol;

pub(crate) fn render_protocol_state(protocol: &Protocol<'_>) -> TokenStream {
    let variants = protocol.states.iter().map(|state| {
        let variant = format_ident!("{}", state.variant);
        let concrete = path_tokens(&state.canonical_path);

        quote! { #variant(#concrete), }
    });

    let conversions = protocol.states.iter().map(|state| {
        let variant = format_ident!("{}", state.variant);
        let concrete = path_tokens(&state.canonical_path);

        quote! {
            impl ::std::convert::From<#concrete> for ProtocolState {
                fn from(state: #concrete) -> Self {
                    ProtocolState::#variant(state)
                }
            }
        }
    });

    quote! {
        pub enum ProtocolState {
            #(#variants)*
        }

        #(#conversions)*
    }
}
