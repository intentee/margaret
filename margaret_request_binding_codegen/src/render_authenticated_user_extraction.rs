use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_context::ExtractionContext;

fn authenticated_user_resolver(
    requirement: AuthenticatedUserRequirement,
    challenge: &AuthenticatedUserChallenge,
) -> TokenStream {
    match requirement {
        AuthenticatedUserRequirement::Optional => quote! {
            margaret::framework::identity::optional_authenticated_user::optional_authenticated_user
        },
        AuthenticatedUserRequirement::Required => match challenge {
            AuthenticatedUserChallenge::Bearer { .. }
            | AuthenticatedUserChallenge::Introspection { .. } => quote! {
                margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user
            },
            AuthenticatedUserChallenge::Session { .. }
            | AuthenticatedUserChallenge::Unchallenged => {
                quote! {
                    margaret::framework::identity::require_authenticated_user::require_authenticated_user
                }
            }
        },
    }
}

pub(crate) fn render_authenticated_user_extraction(
    AuthenticatedUserApplication { challenge, .. }: &AuthenticatedUserApplication,
    requirement: AuthenticatedUserRequirement,
    holder: &Ident,
    ExtractionContext {
        continuation_return,
        error_return,
        provider_access,
        request_local,
    }: &ExtractionContext,
) -> TokenStream {
    let resolver = authenticated_user_resolver(requirement, challenge);

    match challenge {
        AuthenticatedUserChallenge::Session { .. } => quote! {
            let #holder = match #resolver(#holder) {
                margaret::framework::http::requirement::Requirement::Met(value) => value,
                margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
            };
        },
        AuthenticatedUserChallenge::Bearer { .. }
        | AuthenticatedUserChallenge::Introspection { .. }
        | AuthenticatedUserChallenge::Unchallenged => quote! {
            let #holder = match margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(
                    #provider_access.as_ref(),
                    #request_local,
                ).await {
                ::std::result::Result::Ok(outcome) => match #resolver(outcome) {
                    margaret::framework::http::requirement::Requirement::Met(value) => value,
                    margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
                },
                ::std::result::Result::Err(error) => #error_return,
            };
        },
    }
}
