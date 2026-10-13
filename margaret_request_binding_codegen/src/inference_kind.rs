use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InferenceKind {
    Outcome,
    Presence,
}

impl InferenceKind {
    #[must_use]
    pub fn inferred_type(self, user: &TokenStream) -> TokenStream {
        match self {
            Self::Outcome => quote! {
                margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<#user>
            },
            Self::Presence => quote! {
                margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence<#user>
            },
        }
    }
}
