use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_context::ExtractionContext;
use crate::render_model_extraction::render_model_extraction;
use crate::request_binding::RequestBinding;

fn authenticated_user_resolver(
    requirement: AuthenticatedUserRequirement,
    challenge: &AuthenticatedUserChallenge,
) -> TokenStream {
    match (requirement, challenge) {
        (AuthenticatedUserRequirement::Optional, _) => quote! {
            margaret::framework::identity::optional_authenticated_user::optional_authenticated_user
        },
        (
            AuthenticatedUserRequirement::Required,
            AuthenticatedUserChallenge::Bearer { .. }
            | AuthenticatedUserChallenge::Introspection { .. },
        ) => {
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
                        margaret::framework::http::requirement::Requirement::Met(value) => value,
                        margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
                    },
                    ::std::result::Result::Err(error) => #system_error_return,
                };
            }
        }
        RequestBinding::RouteParameterValue { path_key } => quote! {
            let #holder = match margaret::framework::http::require_route_parameter::require_route_parameter(
                #request_local,
                #path_key,
            ) {
                margaret::framework::http::requirement::Requirement::Met(value) => value,
                margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
            };
        },
        RequestBinding::FormRequest { source, extraction } => {
            let inputs_field = source.inputs_field();

            render_model_extraction(
                extraction,
                holder,
                &quote! {
                    margaret::framework::validation::validate::validate(
                        &#request_local.inputs.#inputs_field,
                    )
                },
                continuation_return,
            )
        }
        RequestBinding::PeerSpiffeId => quote! {
            let #holder = match margaret::framework::http::require_peer_spiffe_id::require_peer_spiffe_id(
                #request_local,
            ) {
                margaret::framework::http::requirement::Requirement::Met(value) => value,
                margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
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
        RequestBinding::BearerToken { .. }
        | RequestBinding::IntrospectedBearerToken { .. }
        | RequestBinding::BoundRouteParameter { .. }
        | RequestBinding::FormContent { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::JsonContent { .. }
        | RequestBinding::Next
        | RequestBinding::RequestBodyStream
        | RequestBinding::Routes
        | RequestBinding::UploadedFiles
        | RequestBinding::Views => TokenStream::new(),
    }
}
