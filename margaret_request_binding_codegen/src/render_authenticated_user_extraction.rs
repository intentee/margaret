use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_context::ExtractionContext;
use crate::inference_fallibility::InferenceFallibility;
use crate::inference_kind::InferenceKind;
use crate::user_resolution::UserResolution;

fn required(resolver: &TokenStream, inferred: &TokenStream) -> UserResolution {
    UserResolution::Continued {
        requirement: quote! { #resolver(#inferred) },
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
) -> UserResolution {
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

            required(&resolver, inferred)
        }
        InferenceKind::Presence => match requirement {
            AuthenticatedUserRequirement::Optional => UserResolution::Bound {
                value: quote! {
                    margaret::framework::identity::optional_present_user::optional_present_user(#inferred)
                },
            },
            AuthenticatedUserRequirement::Required => required(
                &quote! { margaret::framework::identity::require_present_user::require_present_user },
                inferred,
            ),
        },
    }
}

pub(crate) fn render_authenticated_user_extraction(
    application: &AuthenticatedUserApplication,
    requirement: AuthenticatedUserRequirement,
    holder: &Ident,
    ExtractionContext {
        error_return,
        provider_access,
        request_local,
        ..
    }: &ExtractionContext,
) -> UserResolution {
    match application.challenge {
        AuthenticatedUserChallenge::Session { .. } => {
            resolved_user(application, requirement, &quote! { #holder })
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

            resolved_user(application, requirement, &inferred)
        }
    }
}
