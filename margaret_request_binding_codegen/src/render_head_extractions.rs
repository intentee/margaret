use proc_macro2::TokenStream;
use quote::quote;

use crate::bound_parameter::BoundParameter;
use crate::captured_providers::CapturedProviders;
use crate::extraction_context::ExtractionContext;
use crate::extraction_phase::ExtractionPhase;
use crate::head_extraction_context::HeadExtractionContext;
use crate::render_authenticated_user_extraction::render_authenticated_user_extraction;
use crate::render_bound_request_extractions::render_bound_request_extractions;
use crate::render_request_extraction::render_request_extraction;
use crate::request_binding::RequestBinding;
use crate::user_resolution::UserResolution;

fn phase_parameters(
    parameters: &[BoundParameter],
    phase: ExtractionPhase,
) -> impl DoubleEndedIterator<Item = &BoundParameter> {
    parameters
        .iter()
        .filter(move |parameter| parameter.binding.extraction_phase() == phase)
}

fn extracted(
    parameter: &BoundParameter,
    captured: &CapturedProviders,
    context: &HeadExtractionContext,
    continued: &TokenStream,
) -> TokenStream {
    let provider_access = captured.access(&parameter.binding, context.owner);
    let extraction_context = ExtractionContext {
        continuation_return: context.continuation_return,
        error_return: context.error_return,
        provider_access: &provider_access,
        request_local: context.request_local,
    };
    let holder = &parameter.holder;

    match &parameter.binding {
        RequestBinding::AuthenticatedUser {
            application,
            requirement,
        } => match render_authenticated_user_extraction(
            application,
            *requirement,
            holder,
            &extraction_context,
        ) {
            UserResolution::Bound { value } => quote! {
                let #holder = #value;
                #continued
            },
            UserResolution::Continued { requirement } => {
                let refused = context.refused;

                quote! {
                    margaret::framework::http::with_requirement::with_requirement(
                        #requirement,
                        #refused,
                        move |#holder| async move { #continued },
                    )
                    .await
                }
            }
        },
        binding => {
            let extraction = render_request_extraction(binding, holder, &extraction_context);

            quote! {
                #extraction
                #continued
            }
        }
    }
}

#[must_use]
pub fn render_head_extractions(
    parameters: &[BoundParameter],
    captured: &CapturedProviders,
    context: &HeadExtractionContext,
    body: &TokenStream,
) -> TokenStream {
    let route_models = render_bound_request_extractions(parameters, captured, context);
    let request_inputs = phase_parameters(parameters, ExtractionPhase::RequestInput)
        .rev()
        .fold(body.clone(), |continued, parameter| {
            extracted(parameter, captured, context, &continued)
        });

    phase_parameters(parameters, ExtractionPhase::CallerIdentity)
        .rev()
        .fold(
            quote! {
                #route_models
                #request_inputs
            },
            |continued, parameter| extracted(parameter, captured, context, &continued),
        )
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::format_ident;
    use quote::quote;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::name_allocator::NameAllocator;

    use super::render_head_extractions;
    use crate::authenticated_user_application::AuthenticatedUserApplication;
    use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
    use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
    use crate::bound_parameter::BoundParameter;
    use crate::captured_providers::CapturedProviders;
    use crate::head_extraction_context::HeadExtractionContext;
    use crate::inference_fallibility::InferenceFallibility;
    use crate::inference_kind::InferenceKind;
    use crate::inference_shape::InferenceShape;
    use crate::request_binding::RequestBinding;
    use crate::route_parameter_lookup::RouteParameterLookup;

    fn route_model() -> BoundParameter {
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
            holder: format_ident!("article"),
        }
    }

    fn rendered(parameters: &[BoundParameter]) -> String {
        let captured = CapturedProviders::capture(parameters, &mut NameAllocator::new());

        render_head_extractions(
            parameters,
            &captured,
            &HeadExtractionContext {
                continuation_return: &quote! { return response },
                error_return: &quote! { return error },
                owner: &TokenStream::new(),
                refused: &quote! { refuse },
                request_local: &format_ident!("request"),
            },
            &quote! { body },
        )
        .to_string()
        .split_whitespace()
        .collect()
    }

    fn renders_before_the_route_model(rendered: &str, caller_identity: &str) -> bool {
        let identification = rendered
            .find(caller_identity)
            .expect("the caller identity is extracted");
        let binding = rendered
            .find("bind_route_parameter(")
            .expect("the route model is bound");

        identification < binding
    }

    #[test]
    fn renders_the_authenticated_user_before_route_models() {
        let user = BoundParameter {
            binding: RequestBinding::AuthenticatedUser {
                application: AuthenticatedUserApplication {
                    challenge: AuthenticatedUserChallenge::Unchallenged,
                    concrete: CanonicalPath::new(vec![
                        "crate".to_string(),
                        "SessionUserProvider".to_string(),
                    ]),
                    field: "session_user_provider".to_string(),
                    inference: InferenceShape {
                        fallibility: InferenceFallibility::Fallible,
                        kind: InferenceKind::Outcome,
                    },
                    injects_peer_spiffe_id: false,
                    injects_routes: false,
                    injects_views: false,
                    model: CanonicalPath::new(vec!["crate".to_string(), "User".to_string()]),
                    wrapper: format_ident!("SessionUserProvider"),
                },
                requirement: AuthenticatedUserRequirement::Required,
            },
            holder: format_ident!("user"),
        };

        assert!(renders_before_the_route_model(
            &rendered(&[route_model(), user]),
            "InfersAuthenticatedUser::infer(",
        ));
    }
}
