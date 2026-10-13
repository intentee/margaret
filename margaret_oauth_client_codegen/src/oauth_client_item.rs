use proc_macro2::TokenStream;
use quote::quote;

use crate::module_sign_in::ModuleSignIn;
use crate::oauth_client_credentials::OAuthClientCredentials;

#[derive(Clone, Copy)]
pub enum OAuthClientItem {
    AuthorizationServerClient,
    ClientCredentials,
    SignInFlow,
    SignInStartHandler,
    TokenExchange,
}

impl OAuthClientItem {
    pub const ALL: [Self; 5] = [
        Self::AuthorizationServerClient,
        Self::ClientCredentials,
        Self::SignInFlow,
        Self::SignInStartHandler,
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
            Self::SignInStartHandler => quote! {
                margaret::framework::oidc_sign_in::sign_in_start_handler::SignInStartHandler
            },
            Self::TokenExchange => quote! {
                margaret::framework::token_exchange_client::token_exchange::TokenExchange
            },
        }
    }

    #[must_use]
    pub fn is_available_to(
        self,
        sign_in: &ModuleSignIn,
        credentials: &OAuthClientCredentials,
    ) -> bool {
        match self {
            Self::ClientCredentials => matches!(credentials, OAuthClientCredentials::Targeted),
            Self::SignInFlow | Self::SignInStartHandler => {
                matches!(sign_in, ModuleSignIn::Available { .. })
            }
            Self::AuthorizationServerClient | Self::TokenExchange => true,
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::AuthorizationServerClient => "AuthorizationServerClient",
            Self::ClientCredentials => "ClientCredentials",
            Self::SignInFlow => "SignInFlow",
            Self::SignInStartHandler => "SignInStartHandler",
            Self::TokenExchange => "TokenExchange",
        }
    }
}
