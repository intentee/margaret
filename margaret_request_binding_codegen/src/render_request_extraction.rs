use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::bearer_token_verifier_field::BEARER_TOKEN_VERIFIER_FIELD;
use crate::extraction_context::ExtractionContext;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_binding::RequestBinding;

fn authenticated_user_resolver(
    requirement: AuthenticatedUserRequirement,
    challenge: &AuthenticatedUserChallenge,
) -> TokenStream {
    match (requirement, challenge) {
        (AuthenticatedUserRequirement::Optional, _) => quote! {
            margaret::framework::identity::optional_authenticated_user::optional_authenticated_user
        },
        (AuthenticatedUserRequirement::Required, AuthenticatedUserChallenge::Bearer { .. }) => {
            quote! {
                margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user
            }
        }
        (AuthenticatedUserRequirement::Required, AuthenticatedUserChallenge::Unchallenged) => {
            quote! {
                margaret::framework::identity::require_authenticated_user::require_authenticated_user
            }
        }
    }
}

#[must_use]
pub fn render_request_extraction(
    binding: &RequestBinding,
    holder: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let continuation_return = context.continuation_return;
    let system_error_return = context.error_return;
    let provider_access = context.provider_access;
    let request_local = context.request_local;
    let error_return = context.response_return;

    match binding {
        RequestBinding::AuthenticatedUser {
            application,
            requirement,
        } => {
            let resolver = authenticated_user_resolver(*requirement, &application.challenge);

            quote! {
                let #holder = match margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(
                        #provider_access.as_ref(),
                        #request_local,
                    ).await {
                    ::std::result::Result::Ok(outcome) => match #resolver(outcome) {
                        ::std::result::Result::Ok(value) => value,
                        ::std::result::Result::Err(response) => #continuation_return,
                    },
                    ::std::result::Result::Err(error) => #system_error_return,
                };
            }
        }
        RequestBinding::BearerToken { claims, .. } => {
            let claims = path_tokens(claims);
            let verifier = format_ident!("{BEARER_TOKEN_VERIFIER_FIELD}");

            quote! {
                let #holder = match margaret::framework::bearer_token_verification::admit_bearer_token::admit_bearer_token::<#claims>(
                    &self.#verifier,
                    #request_local.inputs.server.authorization(),
                )
                .map_err(margaret::framework::anyhow::Error::from)
                {
                    ::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Anonymous) => ::std::option::Option::None,
                    ::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Presented(token)) => ::std::option::Option::Some(token),
                    ::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Refused(response)) => #continuation_return,
                    ::std::result::Result::Err(error) => #system_error_return,
                };
            }
        }
        RequestBinding::RouteParameterValue { path_key } => quote! {
            let #holder = match margaret::framework::http::require_route_parameter::require_route_parameter(
                #request_local,
                #path_key,
            ) {
                Ok(value) => value,
                Err(response) => #error_return,
            };
        },
        RequestBinding::FormRequest { source, extraction } => {
            let variant = source.variant();

            match extraction {
                FormRequestExtraction::Result => quote! {
                    let #holder = margaret::framework::http_validation::validate_input::validate_input(
                        #request_local,
                        margaret::framework::http_validation::request_input::RequestInput::#variant,
                    );
                },
                FormRequestExtraction::Model => quote! {
                    let #holder = match margaret::framework::http_validation::require_input::require_input(
                        #request_local,
                        margaret::framework::http_validation::request_input::RequestInput::#variant,
                    ) {
                        Ok(model) => model,
                        Err(response) => #error_return,
                    };
                },
            }
        }
        RequestBinding::PeerSpiffeId => quote! {
            let #holder = match margaret::framework::http::require_peer_spiffe_id::require_peer_spiffe_id(
                #request_local,
            ) {
                Ok(value) => value,
                Err(response) => #error_return,
            };
        },
        RequestBinding::CurrentRequest => {
            if holder == request_local {
                TokenStream::new()
            } else {
                quote! {
                    let #holder = #request_local;
                }
            }
        }
        RequestBinding::AssetBag => quote! {
            let #holder = ::margaret::framework::asset_bag::asset_bag::AssetBag::new();
        },
        RequestBinding::BoundRouteParameter { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::Next
        | RequestBinding::Routes
        | RequestBinding::Views => TokenStream::new(),
    }
}
