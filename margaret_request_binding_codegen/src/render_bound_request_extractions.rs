use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::bound_parameter::BoundParameter;
use crate::captured_providers::CapturedProviders;
use crate::request_binding::RequestBinding;

struct BoundRouteParameter<'parameter> {
    parameter: &'parameter BoundParameter,
    path_key: &'parameter str,
}

fn joined_future(first: &TokenStream, remaining: &[TokenStream]) -> TokenStream {
    if let Some((second, tail)) = remaining.split_first() {
        let remaining = joined_future(second, tail);

        quote! {
            margaret::framework::route_parameter_binding::join_route_parameter_bindings::join_route_parameter_bindings(
                #first,
                #remaining,
            )
        }
    } else {
        quote! { #first }
    }
}

fn joined_pattern(first: &Ident, remaining: &[&Ident]) -> TokenStream {
    if let Some((second, tail)) = remaining.split_first() {
        let remaining = joined_pattern(second, tail);

        quote! {
            margaret::framework::route_parameter_binding::joined_route_parameter_bindings::JoinedRouteParameterBindings {
                first: #first,
                second: #remaining,
            }
        }
    } else {
        quote! { #first }
    }
}

#[must_use]
pub fn render_bound_request_extractions(
    parameters: &[BoundParameter],
    captured: &CapturedProviders,
    owner: &TokenStream,
    request: &Ident,
    error_return: &TokenStream,
    not_found_return: &TokenStream,
) -> TokenStream {
    let bound: Vec<BoundRouteParameter<'_>> = parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::BoundRouteParameter { path_key, .. } => Some(BoundRouteParameter {
                parameter,
                path_key: path_key.as_str(),
            }),
            _ => None,
        })
        .collect();

    let Some((first, remaining)) = bound.split_first() else {
        return TokenStream::new();
    };

    let binding_future = |bound: &BoundRouteParameter<'_>| {
        let provider = captured.access(&bound.parameter.binding, owner);
        let path_key = bound.path_key;

        quote! {
            margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(
                #request,
                #path_key,
                #provider.as_ref(),
            )
        }
    };
    let first_future = binding_future(first);
    let remaining_futures: Vec<TokenStream> = remaining.iter().map(binding_future).collect();
    let first_holder = &first.parameter.holder;
    let remaining_holders: Vec<&Ident> = remaining
        .iter()
        .map(|bound| &bound.parameter.holder)
        .collect();
    let future = joined_future(&first_future, &remaining_futures);
    let pattern = joined_pattern(first_holder, &remaining_holders);
    let values = std::iter::once(first_holder)
        .chain(remaining_holders.iter().copied())
        .map(|holder| {
            quote! {
                let #holder = match #holder {
                    margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome::Bound(
                        model,
                    ) => model,
                    margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome::NotFound => {
                        #not_found_return
                    }
                };
            }
        });

    quote! {
        let #pattern = match #future.await {
            ::std::result::Result::Ok(outcomes) => outcomes,
            ::std::result::Result::Err(error) => {
                #error_return
            }
        };
        #(#values)*
    }
}
