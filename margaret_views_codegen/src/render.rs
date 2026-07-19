use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::view::View;

pub(crate) fn render(views: &[View]) -> TokenStream {
    let fields = views.iter().map(|view| {
        let name = format_ident!("{}", view.name);
        let concrete = path_tokens(&view.concrete_path);

        quote! { pub #name: ::std::sync::Arc<#concrete>, }
    });

    quote! {
        #[rustfmt::skip]
        pub mod build;

        pub struct Views {
            #(#fields)*
        }
    }
}
