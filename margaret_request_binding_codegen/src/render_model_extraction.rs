use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::form_request_extraction::FormRequestExtraction;

pub(crate) fn render_model_extraction(
    extraction: &FormRequestExtraction,
    holder: &Ident,
    validation: &TokenStream,
    continuation_return: &TokenStream,
) -> TokenStream {
    match extraction {
        FormRequestExtraction::Result => quote! {
            let #holder = #validation;
        },
        FormRequestExtraction::Model => quote! {
            let #holder = match margaret::framework::http_validation::require_input::require_input(
                #validation,
            ) {
                margaret::framework::http::requirement::Requirement::Met(model) => model,
                margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
            };
        },
    }
}
