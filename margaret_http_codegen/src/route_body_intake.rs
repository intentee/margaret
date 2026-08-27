use proc_macro2::TokenStream;
use quote::quote;

use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::http_route::HttpRoute;

pub(crate) fn route_body_intake(route: &HttpRoute) -> TokenStream {
    if route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, RequestBinding::RequestBody))
    {
        quote! { margaret::framework::http::body_intake::BodyIntake::Collected }
    } else {
        quote! { margaret::framework::http::body_intake::BodyIntake::Discarded }
    }
}
