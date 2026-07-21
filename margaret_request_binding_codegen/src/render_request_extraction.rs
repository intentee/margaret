use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::extraction_context::ExtractionContext;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn render_request_extraction(
    binding: &RequestBinding,
    holder: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let request_local = context.request_local;
    let error_return = context.error_return;
    let binder_owner = context.binder_owner;

    match binding {
        RequestBinding::Raw { path_key } => quote! {
            let #holder = match margaret_http::require_route_parameter::require_route_parameter(
                #request_local,
                #path_key,
            ) {
                Ok(value) => value,
                Err(response) => #error_return,
            };
        },
        RequestBinding::Bound {
            binder_field,
            path_key,
            ..
        } => {
            let binder = format_ident!("{}", binder_field);

            quote! {
                let #holder = match margaret_http::require_bound_route_parameter::require_bound_route_parameter(
                    #request_local,
                    #path_key,
                    #binder_owner #binder.as_ref(),
                ).await {
                    Ok(value) => value,
                    Err(response) => #error_return,
                };
            }
        }
        RequestBinding::FormRequest { source, extraction } => {
            let variant = source.variant();

            match extraction {
                FormRequestExtraction::Result => quote! {
                    let #holder = margaret_http_validation::validate_input::validate_input(
                        #request_local,
                        margaret_http_validation::request_input::RequestInput::#variant,
                    );
                },
                FormRequestExtraction::Model => quote! {
                    let #holder = match margaret_http_validation::require_input::require_input(
                        #request_local,
                        margaret_http_validation::request_input::RequestInput::#variant,
                    ) {
                        Ok(model) => model,
                        Err(response) => #error_return,
                    };
                },
            }
        }
        RequestBinding::PeerSpiffeId => quote! {
            let #holder = match margaret_http::require_peer_spiffe_id::require_peer_spiffe_id(
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
            let #holder = ::margaret_asset_bag::asset_bag::AssetBag::new();
        },
        RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::Next
        | RequestBinding::Routes
        | RequestBinding::Views => TokenStream::new(),
    }
}
