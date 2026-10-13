use proc_macro2::TokenStream;
use quote::quote;

use crate::spiffe_activation::SpiffeActivation;

pub(crate) fn svid_bundle_type(
    SpiffeActivation {
        client_active,
        server_active,
    }: SpiffeActivation,
) -> TokenStream {
    match (server_active, client_active) {
        (true, true) => quote! { margaret::framework::spiffe_svid_bundle::svid_bundle::SvidBundle },
        (true, false) => {
            quote! { margaret::framework::spiffe_svid_server::svid_server_bundle::SvidServerBundle }
        }
        (false, true | false) => {
            quote! { margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle }
        }
    }
}
