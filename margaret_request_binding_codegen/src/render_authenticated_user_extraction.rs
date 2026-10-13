use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_context::ExtractionContext;
use crate::inference_fallibility::InferenceFallibility;
use crate::inference_kind::InferenceKind;

fn required(
    resolver: &TokenStream,
    inferred: &TokenStream,
    continuation_return: &TokenStream,
) -> TokenStream {
    quote! {
        match #resolver(#inferred) {
            margaret::framework::http::requirement::Requirement::Met(value) => value,
            margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
        }
    }
}

fn resolved_user(
    AuthenticatedUserApplication {
        challenge,
        inference,
        ..
    }: &AuthenticatedUserApplication,
    requirement: AuthenticatedUserRequirement,
    inferred: &TokenStream,
    continuation_return: &TokenStream,
) -> TokenStream {
    match inference.kind {
        InferenceKind::Outcome => {
            let resolver = match requirement {
                AuthenticatedUserRequirement::Optional => quote! {
                    margaret::framework::identity::optional_authenticated_user::optional_authenticated_user
                },
                AuthenticatedUserRequirement::Required => match challenge {
                    AuthenticatedUserChallenge::Bearer { .. }
                    | AuthenticatedUserChallenge::Introspection { .. } => quote! {
                        margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user
                    },
                    AuthenticatedUserChallenge::Session { .. }
                    | AuthenticatedUserChallenge::Unchallenged => quote! {
                        margaret::framework::identity::require_authenticated_user::require_authenticated_user
                    },
                },
            };

            required(&resolver, inferred, continuation_return)
        }
        InferenceKind::Presence => match requirement {
            AuthenticatedUserRequirement::Optional => quote! {
                margaret::framework::identity::optional_present_user::optional_present_user(#inferred)
            },
            AuthenticatedUserRequirement::Required => required(
                &quote! { margaret::framework::identity::require_present_user::require_present_user },
                inferred,
                continuation_return,
            ),
        },
    }
}

pub(crate) fn render_authenticated_user_extraction(
    application: &AuthenticatedUserApplication,
    requirement: AuthenticatedUserRequirement,
    holder: &Ident,
    ExtractionContext {
        continuation_return,
        error_return,
        provider_access,
        request_local,
    }: &ExtractionContext,
) -> TokenStream {
    match application.challenge {
        AuthenticatedUserChallenge::Session { .. } => {
            let resolved = resolved_user(
                application,
                requirement,
                &quote! { #holder },
                continuation_return,
            );

            quote! { let #holder = #resolved; }
        }
        AuthenticatedUserChallenge::Bearer { .. }
        | AuthenticatedUserChallenge::Introspection { .. }
        | AuthenticatedUserChallenge::Unchallenged => {
            let infer = quote! {
                margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(
                    #provider_access.as_ref(),
                    #request_local,
                ).await
            };
            let inferred = match application.inference.fallibility {
                InferenceFallibility::Fallible => quote! {
                    match #infer {
                        ::std::result::Result::Ok(inferred) => inferred,
                        ::std::result::Result::Err(error) => #error_return,
                    }
                },
                InferenceFallibility::Infallible => infer,
            };
            let resolved = resolved_user(application, requirement, &inferred, continuation_return);

            quote! { let #holder = #resolved; }
        }
    }
}
