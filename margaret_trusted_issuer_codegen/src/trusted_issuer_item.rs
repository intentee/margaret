use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum TrustedIssuerItem {
    IssuerKeySet,
    IssuerMetadata,
    PolledKeySet,
    TrustedIssuer,
}

impl TrustedIssuerItem {
    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::IssuerKeySet => quote! {
                margaret::framework::issuer_key_set::issuer_key_set::IssuerKeySet
            },
            Self::IssuerMetadata => quote! {
                margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata
            },
            Self::PolledKeySet => quote! {
                margaret::framework::issuer_directory::polled_key_set::PolledKeySet
            },
            Self::TrustedIssuer => quote! {
                margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer
            },
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::IssuerKeySet => "IssuerKeySet",
            Self::IssuerMetadata => "IssuerMetadata",
            Self::PolledKeySet => "PolledKeySet",
            Self::TrustedIssuer => "TrustedIssuer",
        }
    }
}
