use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub enum OAuthClientItem {
    AuthorizationServerClient,
    ClientCredentials,
    SignInFlow,
    TokenExchange,
}

impl OAuthClientItem {
    pub const ALL: [Self; 4] = [
        Self::AuthorizationServerClient,
        Self::ClientCredentials,
        Self::SignInFlow,
        Self::TokenExchange,
    ];

    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::AuthorizationServerClient => quote! {
                margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient
            },
            Self::ClientCredentials => quote! {
                margaret::framework::client_credentials::client_credentials::ClientCredentials
            },
            Self::SignInFlow => quote! {
                margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow
            },
            Self::TokenExchange => quote! {
                margaret::framework::token_exchange_client::token_exchange::TokenExchange
            },
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::AuthorizationServerClient => "AuthorizationServerClient",
            Self::ClientCredentials => "ClientCredentials",
            Self::SignInFlow => "SignInFlow",
            Self::TokenExchange => "TokenExchange",
        }
    }
}
