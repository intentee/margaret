use proc_macro2::TokenStream;
use quote::quote;

use crate::forward_targets_layout::ForwardTargetsLayout;

pub(crate) enum ServerForwarding {
    Targets(ForwardTargetsLayout),
    Unforwarded,
}

impl ServerForwarding {
    pub(crate) fn target_type(&self, routes_module: &TokenStream) -> TokenStream {
        match self {
            Self::Targets(ForwardTargetsLayout { target_type, .. }) => {
                quote! { #routes_module::forward_targets::#target_type }
            }
            Self::Unforwarded => quote! { ::core::convert::Infallible },
        }
    }
}
