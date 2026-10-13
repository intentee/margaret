use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

fn with_route_parameters(
    parameters: &[BoundParameter],
    request_local: &Ident,
    refused: &TokenStream,
    body: &TokenStream,
) -> TokenStream {
    let routed: Vec<RoutedParameter<'_>> = parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::RouteParameterValue { path_key, .. }
            | RequestBinding::BoundRouteParameter { path_key, .. } => Some(RoutedParameter {
                holder: &parameter.holder,
                path_key,
            }),
            _ => None,
        })
        .collect();

    if routed.is_empty() {
        return body.clone();
    }

    let names = routed
        .iter()
        .map(|RoutedParameter { path_key, .. }| path_key);
    let holders = routed.iter().map(|RoutedParameter { holder, .. }| holder);

    quote! {
        margaret::framework::http::with_route_parameters::with_route_parameters(
            #request_local,
            [#(#names),*],
            #refused,
            move |[#(#holders),*]| async move { #body },
        )
        .await
    }
}

struct RoutedParameter<'parameter> {
    holder: &'parameter Ident,
    path_key: &'parameter str,
}

#[must_use]
pub fn render_route_inputs(
    parameters: &[BoundParameter],
    request_local: &Ident,
    refused: &TokenStream,
    body: &TokenStream,
) -> TokenStream {
    let routed = with_route_parameters(parameters, request_local, refused, body);

    match parameters
        .iter()
        .find(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
    {
        Some(BoundParameter { holder, .. }) => quote! {
            margaret::framework::http::with_verified_peer::with_verified_peer(
                #request_local,
                #refused,
                move |#holder| async move { #routed },
            )
            .await
        },
        None => routed,
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use quote::quote;

    use margaret_attributes::canonical_path::CanonicalPath;

    use super::render_route_inputs;
    use crate::bound_parameter::BoundParameter;
    use crate::request_binding::RequestBinding;
    use crate::route_parameter_conversion::RouteParameterConversion;
    use crate::route_parameter_lookup::RouteParameterLookup;

    fn rendered(parameters: &[BoundParameter]) -> String {
        render_route_inputs(
            parameters,
            &format_ident!("request"),
            &quote! { Ok },
            &quote! { body },
        )
        .to_string()
        .split_whitespace()
        .collect()
    }

    fn topic() -> BoundParameter {
        BoundParameter {
            binding: RequestBinding::RouteParameterValue {
                conversion: RouteParameterConversion::Identity,
                path_key: "topic".to_string(),
            },
            holder: format_ident!("argument_1"),
        }
    }

    fn article() -> BoundParameter {
        BoundParameter {
            binding: RequestBinding::BoundRouteParameter {
                binder_field: "article_store".to_string(),
                binder_provider: CanonicalPath::new(vec![
                    "crate".to_string(),
                    "ArticleStore".to_string(),
                ]),
                lookup: RouteParameterLookup::Binder,
                path_key: "article".to_string(),
            },
            holder: format_ident!("argument_2"),
        }
    }

    fn peer() -> BoundParameter {
        BoundParameter {
            binding: RequestBinding::PeerSpiffeId,
            holder: format_ident!("argument_3"),
        }
    }

    #[test]
    fn leaves_a_body_without_route_inputs_as_it_is() {
        assert_eq!(
            render_route_inputs(
                &[BoundParameter {
                    binding: RequestBinding::CurrentRequest,
                    holder: format_ident!("argument_1"),
                }],
                &format_ident!("request"),
                &quote! { Ok },
                &quote! { body },
            )
            .to_string(),
            quote! { body }.to_string()
        );
    }

    #[test]
    fn hands_every_route_parameter_to_the_body_in_declared_order() {
        assert_eq!(
            rendered(&[topic(), article()]),
            "margaret::framework::http::with_route_parameters::with_route_parameters(request,[\"topic\",\"article\"],Ok,move|[argument_1,argument_2]|asyncmove{body},).await"
        );
    }

    #[test]
    fn verifies_the_peer_before_resolving_route_parameters() {
        assert_eq!(
            rendered(&[topic(), peer()]),
            "margaret::framework::http::with_verified_peer::with_verified_peer(request,Ok,move|argument_3|asyncmove{margaret::framework::http::with_route_parameters::with_route_parameters(request,[\"topic\"],Ok,move|[argument_1]|asyncmove{body},).await},).await"
        );
    }
}
