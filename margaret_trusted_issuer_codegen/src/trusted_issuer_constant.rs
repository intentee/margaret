use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum TrustedIssuerConstant {
    DiscoveredIssuer,
    JwksEndpointIssuer,
    TokenTrust,
}

impl TrustedIssuerConstant {
    #[must_use]
    pub fn constant_name(self) -> &'static str {
        match self {
            Self::DiscoveredIssuer => "DISCOVERED_ISSUER",
            Self::JwksEndpointIssuer => "JWKS_ENDPOINT_ISSUER",
            Self::TokenTrust => "TOKEN_TRUST",
        }
    }

    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::DiscoveredIssuer => quote! {
                margaret::framework::issuer_directory::discovered_issuer::DiscoveredIssuer
            },
            Self::JwksEndpointIssuer => quote! {
                margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer
            },
            Self::TokenTrust => quote! {
                margaret::framework::token_trust::token_trust::TokenTrust
            },
        }
    }

    #[must_use]
    pub fn module_name(self) -> &'static str {
        match self {
            Self::DiscoveredIssuer => "discovered_issuer",
            Self::JwksEndpointIssuer => "jwks_endpoint_issuer",
            Self::TokenTrust => "token_trust",
        }
    }
}
