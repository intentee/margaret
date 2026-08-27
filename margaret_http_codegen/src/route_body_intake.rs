use proc_macro2::TokenStream;
use quote::quote;

use margaret_request_binding_codegen::request_body_intake::RequestBodyIntake;

pub(crate) fn route_body_intake(intake: RequestBodyIntake) -> TokenStream {
    match intake {
        RequestBodyIntake::Collected => {
            quote! { margaret::framework::http::body_intake::BodyIntake::Collected }
        }
        RequestBodyIntake::Ignored => {
            quote! { margaret::framework::http::body_intake::BodyIntake::Ignored }
        }
        RequestBodyIntake::Parsed => {
            quote! { margaret::framework::http::body_intake::BodyIntake::Parsed }
        }
    }
}
