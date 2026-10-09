use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::bound_parameter::BoundParameter;
use crate::captured_providers::CapturedProviders;
use crate::head_extraction_context::HeadExtractionContext;

#[must_use]
pub fn render_session_user_inference(
    BoundParameter { binding, holder }: &BoundParameter,
    captured: &CapturedProviders,
    cookie_changes: &Ident,
    HeadExtractionContext {
        error_return,
        owner,
        request_local,
        ..
    }: &HeadExtractionContext,
) -> TokenStream {
    let provider_access = captured.access(binding, owner);

    quote! {
        let margaret::framework::identity::session_user_inference::SessionUserInference {
            cookie_changes: #cookie_changes,
            outcome: #holder,
        } = match margaret::framework::identity::infers_session_user::InfersSessionUser::infer(
                #provider_access.as_ref(),
                #request_local,
            ).await {
            ::std::result::Result::Ok(inference) => inference,
            ::std::result::Result::Err(error) => #error_return,
        };
    }
}
