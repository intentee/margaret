use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum AcceptedClientItem {
    IssuerKeySet,
    PolledKeySet,
    RegisteredClient,
}

impl AcceptedClientItem {
    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::IssuerKeySet => quote! {
                margaret::framework::issuer_key_set::issuer_key_set::IssuerKeySet
            },
            Self::PolledKeySet => quote! {
                margaret::framework::issuer_directory::polled_key_set::PolledKeySet
            },
            Self::RegisteredClient => quote! {
                margaret::framework::accepted_clients::registered_client::RegisteredClient
            },
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::IssuerKeySet => "IssuerKeySet",
            Self::PolledKeySet => "PolledKeySet",
            Self::RegisteredClient => "RegisteredClient",
        }
    }
}
