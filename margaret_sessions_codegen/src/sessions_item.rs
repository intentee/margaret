use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionsItem {
    ConsumedSessions,
    IssuedSessions,
    SessionRefreshEndpoint,
    SessionSignOutEndpoint,
}

impl SessionsItem {
    #[must_use]
    pub fn framework_path(self) -> TokenStream {
        match self {
            Self::ConsumedSessions => quote! {
                margaret::framework::sessions::consumed_sessions::ConsumedSessions
            },
            Self::IssuedSessions => quote! {
                margaret::framework::sessions::issued_sessions::IssuedSessions
            },
            Self::SessionRefreshEndpoint => quote! {
                margaret::framework::sessions::session_refresh_endpoint::SessionRefreshEndpoint
            },
            Self::SessionSignOutEndpoint => quote! {
                margaret::framework::sessions::session_sign_out_endpoint::SessionSignOutEndpoint
            },
        }
    }

    #[must_use]
    pub fn type_name(self) -> &'static str {
        match self {
            Self::ConsumedSessions => "ConsumedSessions",
            Self::IssuedSessions => "IssuedSessions",
            Self::SessionRefreshEndpoint => "SessionRefreshEndpoint",
            Self::SessionSignOutEndpoint => "SessionSignOutEndpoint",
        }
    }
}
