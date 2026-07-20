use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::alphabet_ident::alphabet_ident;
use crate::websocket_transition::WebsocketTransition;

pub(crate) fn render_transition_alphabet(transition: &WebsocketTransition) -> TokenStream {
    let alphabet = alphabet_ident(transition);

    let variants = transition.emits.iter().map(|emit| {
        let variant = format_ident!("{}", emit.variant);
        let concrete = path_tokens(&emit.canonical_path);

        quote! { #variant(#concrete), }
    });

    let conversions = transition.emits.iter().map(|emit| {
        let variant = format_ident!("{}", emit.variant);
        let concrete = path_tokens(&emit.canonical_path);

        quote! {
            impl ::std::convert::From<#concrete> for #alphabet {
                fn from(message: #concrete) -> Self {
                    #alphabet::#variant(message)
                }
            }
        }
    });

    let wire_method_body = if transition.emits.is_empty() {
        quote! { match *self {} }
    } else {
        let arms = transition.emits.iter().map(|emit| {
            let variant = format_ident!("{}", emit.variant);

            quote! {
                #alphabet::#variant(message) => {
                    margaret_websocket::websocket_outbound::WebsocketOutbound::wire_method(message)
                }
            }
        });

        quote! { match self { #(#arms)* } }
    };

    quote! {
        #[derive(::serde::Serialize)]
        #[serde(untagged)]
        pub enum #alphabet {
            #(#variants)*
        }

        #(#conversions)*

        impl margaret_websocket::websocket_outbound::WebsocketOutbound for #alphabet {
            fn wire_method(&self) -> &'static str {
                #wire_method_body
            }
        }
    }
}
