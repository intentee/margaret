use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::extraction_context::ExtractionContext;
use crate::render_model_extraction::render_model_extraction;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn render_request_extraction(
    binding: &RequestBinding,
    holder: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let continuation_return = context.continuation_return;
    let request_local = context.request_local;

    match binding {
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
        RequestBinding::CurrentRequest => quote! {
            let #holder = #request_local;
        },
        RequestBinding::AssetBag => quote! {
            let #holder = ::margaret::framework::asset_bag::asset_bag::AssetBag::new();
        },
        RequestBinding::AuthenticatedUser { .. }
        | RequestBinding::BearerToken { .. }
        | RequestBinding::IntrospectedBearerToken { .. }
        | RequestBinding::BoundRouteParameter { .. }
        | RequestBinding::FormContent { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::JsonContent { .. }
        | RequestBinding::Next
        | RequestBinding::RequestBodyStream
        | RequestBinding::Routes
        | RequestBinding::Session { .. }
        | RequestBinding::UploadedFiles
        | RequestBinding::Views => TokenStream::new(),
    }
}
