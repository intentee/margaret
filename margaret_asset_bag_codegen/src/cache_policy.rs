use proc_macro2::TokenStream;
use quote::quote;

pub(crate) enum CachePolicy {
    Immutable,
    Revalidate,
}

impl CachePolicy {
    pub(crate) fn response_application(&self) -> TokenStream {
        match self {
            Self::Immutable => quote! { .immutable_asset() },
            Self::Revalidate => quote! { .revalidating_asset() },
        }
    }
}
