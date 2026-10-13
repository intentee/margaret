use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::routes_ident::routes_ident;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

pub(crate) enum ServeInvocationField {
    Arguments {
        arguments_type: TokenStream,
        literal: TokenStream,
    },
    Listeners {
        listeners: Vec<Ident>,
    },
    Routes,
    SpiffeBundle {
        bundle_type: TokenStream,
    },
}

impl ServeInvocationField {
    pub(crate) fn declaration(&self) -> TokenStream {
        let name = self.name();
        let field_type = match self {
            Self::Arguments { arguments_type, .. } => arguments_type.clone(),
            Self::Listeners { listeners } => {
                let count = Literal::usize_unsuffixed(listeners.len());

                quote! { [margaret::framework::service::server_listener::ServerListener; #count] }
            }
            Self::Routes => {
                let routes = path_tokens(&umbrella_item_path(&["routes", "Routes"]));

                quote! { ::std::sync::Arc<#routes> }
            }
            Self::SpiffeBundle { bundle_type } => bundle_type.clone(),
        };

        quote! { pub #name: #field_type, }
    }

    pub(crate) fn initialization(&self) -> TokenStream {
        let name = self.name();

        match self {
            Self::Arguments { literal, .. } => quote! { #name: #literal, },
            Self::Listeners { listeners } => quote! { #name: [#(#listeners),*], },
            Self::Routes | Self::SpiffeBundle { .. } => quote! { #name, },
        }
    }

    pub(crate) fn name(&self) -> Ident {
        match self {
            Self::Arguments { .. } => format_ident!("arguments"),
            Self::Listeners { .. } => format_ident!("listeners"),
            Self::Routes => routes_ident(),
            Self::SpiffeBundle { .. } => format_ident!("spiffe_bundle"),
        }
    }
}
