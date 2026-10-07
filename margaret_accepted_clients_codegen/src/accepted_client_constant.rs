use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum AcceptedClientConstant {
    AcceptedClient,
    ConfidentialPrivileges,
    JwksEndpointIssuer,
}

impl AcceptedClientConstant {
    #[must_use]
    pub fn constant_name(self) -> &'static str {
        match self {
            Self::AcceptedClient => "ACCEPTED_CLIENT",
            Self::ConfidentialPrivileges => "CONFIDENTIAL_PRIVILEGES",
            Self::JwksEndpointIssuer => "JWKS_ENDPOINT_ISSUER",
        }
    }

    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::AcceptedClient => quote! {
                margaret::framework::accepted_clients::accepted_client::AcceptedClient
            },
            Self::ConfidentialPrivileges => quote! {
                margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges
            },
            Self::JwksEndpointIssuer => quote! {
                margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer
            },
        }
    }

    #[must_use]
    pub fn module_name(self) -> &'static str {
        match self {
            Self::AcceptedClient => "accepted_client",
            Self::ConfidentialPrivileges => "confidential_privileges",
            Self::JwksEndpointIssuer => "jwks_endpoint_issuer",
        }
    }
}
