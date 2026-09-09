use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_context::ExtractionContext;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_binding::RequestBinding;

fn authenticated_user_resolver(requirement: AuthenticatedUserRequirement) -> TokenStream {
    match requirement {
        AuthenticatedUserRequirement::Optional => quote! {
            margaret::framework::identity::optional_authenticated_user::optional_authenticated_user
        },
        AuthenticatedUserRequirement::Required => quote! {
            margaret::framework::identity::require_authenticated_user::require_authenticated_user
        },
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
        RequestBinding::AuthenticatedUser { requirement, .. } => {
            let resolver = authenticated_user_resolver(*requirement);

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
        RequestBinding::RequestBody => quote! {
            let #holder = #request_local.inputs.body.collected();
        },
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
