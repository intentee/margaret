use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn render_oidc_token_extractions(
    parameters: &[BoundParameter],
    presented_bearer: &Ident,
    request: &Ident,
    error_return: &TokenStream,
) -> TokenStream {
    let verifications = parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::OidcToken { claims, verifier } => {
                let holder = &parameter.holder;
                let claims = path_tokens(claims);
                let verifier = format_ident!("{}", verifier.field);

                Some(quote! {
                    let #holder = self.#verifier.verify::<#claims>(&#presented_bearer);
                })
            }
            _ => None,
        });

    quote! {
        let #presented_bearer = match margaret::framework::oidc_client::presented_bearer::PresentedBearer::read(
            #request.inputs.server.authorization(),
        )
        .map_err(margaret::framework::anyhow::Error::from)
        {
            ::std::result::Result::Ok(presented_bearer) => presented_bearer,
            ::std::result::Result::Err(error) => #error_return,
        };
        #(#verifications)*
    }
}
