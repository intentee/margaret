use std::iter;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::bound_parameter::BoundParameter;
use crate::captured_providers::CapturedProviders;
use crate::head_extraction_context::HeadExtractionContext;
use crate::request_binding::RequestBinding;
use crate::route_parameter_lookup::RouteParameterLookup;

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

struct BoundRouteParameter<'parameter> {
    lookup: &'parameter RouteParameterLookup,
    parameter: &'parameter BoundParameter,
    path_key: &'parameter str,
}

pub(crate) fn render_bound_request_extractions(
    parameters: &[BoundParameter],
    captured: &CapturedProviders,
    HeadExtractionContext {
        continuation_return,
        owner,
        request_local,
        ..
    }: &HeadExtractionContext,
) -> TokenStream {
    let bound: Vec<BoundRouteParameter<'_>> = parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::BoundRouteParameter {
                lookup, path_key, ..
            } => Some(BoundRouteParameter {
                lookup,
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
        let binder = match bound.lookup {
            RouteParameterLookup::Binder => quote! { #provider.as_ref() },
            RouteParameterLookup::PrimaryKey { loaded } => {
                let loaded = path_tokens(loaded);

                quote! {
                    &margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder::<#loaded>::new(
                        #provider.as_ref(),
                    )
                }
            }
        };

        quote! {
            margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(
                #request_local,
                #path_key,
                #binder,
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
    let values = iter::once(first_holder)
        .chain(remaining_holders.iter().copied())
        .map(|holder| {
            quote! {
                let #holder = match #holder {
                    margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome::Bound(
                        model,
                    ) => model,
                    margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome::NotFound => {
                        let response = margaret::framework::http::response_continuation::ResponseContinuation::from(
                            margaret::framework::http::response::Response::not_found(),
                        );

                        #continuation_return
                    }
                };
            }
        });

    quote! {
        let #pattern = match #future.await {
            ::std::result::Result::Ok(outcomes) => outcomes,
            ::std::result::Result::Err(error) => {
                return ::std::result::Result::Err(error.into())
            }
        };
        #(#values)*
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::format_ident;
    use quote::quote;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::name_allocator::NameAllocator;

    use super::render_bound_request_extractions;
    use crate::bound_parameter::BoundParameter;
    use crate::captured_providers::CapturedProviders;
    use crate::head_extraction_context::HeadExtractionContext;
    use crate::request_binding::RequestBinding;
    use crate::route_parameter_lookup::RouteParameterLookup;

    #[test]
    fn binds_a_route_model_by_its_primary_key_through_the_database() {
        let parameters = [BoundParameter {
            binding: RequestBinding::BoundRouteParameter {
                binder_field: "framework_database".to_string(),
                binder_provider: CanonicalPath::new(vec![
                    "margaret".to_string(),
                    "Database".to_string(),
                ]),
                lookup: RouteParameterLookup::PrimaryKey {
                    loaded: CanonicalPath::new(vec!["crate".to_string(), "Article".to_string()]),
                },
                path_key: "article".to_string(),
            },
            holder: format_ident!("article"),
        }];
        let captured = CapturedProviders::capture(&parameters, &mut NameAllocator::new());
        let rendered: String = render_bound_request_extractions(
            &parameters,
            &captured,
            &HeadExtractionContext {
                continuation_return: &quote! { return response },
                cookie_changes: &format_ident!("changed_cookies"),
                error_return: &quote! { return error },
                owner: &TokenStream::new(),
                request_local: &format_ident!("request"),
            },
        )
        .to_string()
        .split_whitespace()
        .collect();

        assert!(rendered.contains(
            "require_bound_route_parameter(request,\"article\",&margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder::<crate::Article>::new(framework_database.as_ref(),),)"
        ));
    }
}
