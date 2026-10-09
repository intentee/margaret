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

fn phase_extractions(
    parameters: &[BoundParameter],
    captured: &CapturedProviders,
    context: &HeadExtractionContext,
    phase: ExtractionPhase,
) -> TokenStream {
    let extractions = parameters
        .iter()
        .filter(|parameter| parameter.binding.extraction_phase() == phase)
        .map(|parameter| {
            let extraction_context = ExtractionContext {
                continuation_return: context.continuation_return,
                error_return: context.error_return,
                provider_access: &captured.access(&parameter.binding, context.owner),
                request_local: context.request_local,
            };

            match &parameter.binding {
                RequestBinding::AuthenticatedUser {
                    application,
                    requirement,
                } => render_authenticated_user_extraction(
                    application,
                    *requirement,
                    &parameter.holder,
                    context.cookie_changes,
                    &extraction_context,
                ),
                _ => render_request_extraction(
                    &parameter.binding,
                    &parameter.holder,
                    &extraction_context,
                ),
            }
        });

    quote! { #(#extractions)* }
}

#[must_use]
pub fn render_head_extractions(
    parameters: &[BoundParameter],
    captured: &CapturedProviders,
    context: &HeadExtractionContext,
) -> TokenStream {
    let caller_identity = phase_extractions(
        parameters,
        captured,
        context,
        ExtractionPhase::CallerIdentity,
    );
    let route_models = render_bound_request_extractions(parameters, captured, context);
    let request_inputs =
        phase_extractions(parameters, captured, context, ExtractionPhase::RequestInput);

    quote! {
        #caller_identity
        #route_models
        #request_inputs
    }
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
                cookie_changes: &format_ident!("changed_cookies"),
                error_return: &quote! { return error },
                owner: &TokenStream::new(),
                request_local: &format_ident!("request"),
            },
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
            .find("require_bound_route_parameter(")
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

    #[test]
    fn renders_the_peer_identity_before_route_models() {
        let peer = BoundParameter {
            binding: RequestBinding::PeerSpiffeId,
            holder: format_ident!("peer"),
        };

        assert!(renders_before_the_route_model(
            &rendered(&[route_model(), peer]),
            "require_peer_spiffe_id(",
        ));
    }
}
