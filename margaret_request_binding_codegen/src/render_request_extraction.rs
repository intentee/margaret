use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::extraction_context::ExtractionContext;
use crate::render_model_extraction::render_model_extraction;
use crate::request_binding::RequestBinding;
use crate::route_parameter_conversion::RouteParameterConversion;

#[must_use]
pub fn render_request_extraction(
    binding: &RequestBinding,
    holder: &Ident,
    context: &ExtractionContext,
) -> TokenStream {
    let continuation_return = context.continuation_return;
    let request_local = context.request_local;

    match binding {
        RequestBinding::RouteParameterValue { conversion, .. } => match conversion {
            RouteParameterConversion::Fallible => quote! {
                let #holder = match margaret::framework::http::route_parameter_value::route_parameter_value(
                    #holder,
                ) {
                    margaret::framework::http::requirement::Requirement::Met(value) => value,
                    margaret::framework::http::requirement::Requirement::Unmet(response) => #continuation_return,
                };
            },
            RouteParameterConversion::Identity => TokenStream::new(),
            RouteParameterConversion::Infallible => quote! {
                let #holder = ::std::convert::From::from(#holder);
            },
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
        RequestBinding::CurrentRequest => quote! {
            let #holder = #request_local;
        },
        RequestBinding::AssetBag => quote! {
            let #holder = ::margaret::framework::asset_bag::asset_bag::AssetBag::new();
        },
        RequestBinding::AuthenticatedUser { .. }
        | RequestBinding::PeerSpiffeId
        | RequestBinding::BearerToken { .. }
        | RequestBinding::IntrospectedBearerToken { .. }
        | RequestBinding::BoundRouteParameter { .. }
        | RequestBinding::FormContent { .. }
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

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::format_ident;
    use quote::quote;

    use super::render_request_extraction;
    use crate::extraction_context::ExtractionContext;
    use crate::request_binding::RequestBinding;
    use crate::route_parameter_conversion::RouteParameterConversion;

    fn extracted(conversion: RouteParameterConversion) -> String {
        render_request_extraction(
            &RequestBinding::RouteParameterValue {
                conversion,
                path_key: "slug".to_string(),
            },
            &format_ident!("argument_1"),
            &ExtractionContext {
                continuation_return: &quote! { return response },
                error_return: &quote! { return error },
                provider_access: &TokenStream::new(),
                request_local: &format_ident!("request"),
            },
        )
        .to_string()
        .split_whitespace()
        .collect()
    }

    #[test]
    fn hands_a_string_route_parameter_over_as_it_is() {
        assert_eq!(extracted(RouteParameterConversion::Identity), "");
    }

    #[test]
    fn converts_a_value_built_from_any_string_without_a_refusal() {
        assert_eq!(
            extracted(RouteParameterConversion::Infallible),
            "letargument_1=::std::convert::From::from(argument_1);"
        );
    }

    #[test]
    fn answers_a_value_its_type_rejects_with_the_continuation() {
        assert_eq!(
            extracted(RouteParameterConversion::Fallible),
            "letargument_1=matchmargaret::framework::http::route_parameter_value::route_parameter_value(argument_1,){margaret::framework::http::requirement::Requirement::Met(value)=>value,margaret::framework::http::requirement::Requirement::Unmet(response)=>returnresponse,};"
        );
    }
}
