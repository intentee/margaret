use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_http_codegen::framework_input::FrameworkInput;
use margaret_http_codegen::framework_responder::FrameworkResponder;
use margaret_http_codegen::framework_responders::FrameworkResponders;
use margaret_oauth_client_codegen::oauth_client_item::OAuthClientItem;
use margaret_oauth_client_codegen::oauth_client_item_path::oauth_client_item_path;
use margaret_oauth_client_codegen::oauth_client_sign_in_callback_handler_path::oauth_client_sign_in_callback_handler_path;
use margaret_oidc_provider_codegen::declared_consent::DeclaredConsent;
use margaret_oidc_provider_codegen::declared_consent_route::DeclaredConsentRoute;
use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
use margaret_sign_in_endpoints_codegen::client_sign_in::ClientSignIn;
use margaret_sign_in_endpoints_codegen::served_sign_in::ServedSignIn;
use margaret_sign_in_endpoints_codegen::sign_in_service::SignInService;

use crate::codegen_error::CodegenError;
use crate::served_endpoints::ServedEndpoints;

fn handled_routes(
    ServedEndpoints {
        routes, sessions, ..
    }: &ServedEndpoints,
    client_sign_ins: &[ClientSignIn],
) -> Vec<HandledRoute> {
    let mut handled: Vec<HandledRoute> = routes
        .routes
        .iter()
        .map(|route| HandledRoute {
            handler: route.endpoint.handler_path(),
            input: route.input,
            route: route.route.clone(),
        })
        .collect();

    if let DeclaredConsent::Declared(DeclaredConsentRoute { input, route, .. }) = &routes.consent {
        handled.push(HandledRoute {
            handler: ProviderEndpoint::Consent.handler_path(),
            input: *input,
            route: route.clone(),
        });
    }

    handled.extend(sessions.routes.iter().map(|route| HandledRoute {
        handler: route.endpoint.handler_path(),
        input: route.endpoint.input(),
        route: route.route.clone(),
    }));

    for ClientSignIn { binding, service } in client_sign_ins {
        if let SignInService::Served(ServedSignIn {
            callback, start, ..
        }) = service
        {
            let tag = &binding.client.tag;

            handled.push(HandledRoute {
                handler: oauth_client_item_path(tag, OAuthClientItem::SignInStartHandler),
                input: FrameworkInput::Head,
                route: start.clone(),
            });
            handled.push(HandledRoute {
                handler: oauth_client_sign_in_callback_handler_path(tag),
                input: FrameworkInput::Head,
                route: callback.route.clone(),
            });
        }
    }

    handled
}

fn planned_responders(
    bindings: &ContainerBindings,
    handled_routes: Vec<HandledRoute>,
) -> Result<FrameworkResponders, CodegenError> {
    let mut responders = BTreeMap::new();

    for HandledRoute {
        handler,
        input,
        route,
    } in handled_routes
    {
        let binding = bindings.provider(&handler).ok_or_else(|| {
            CodegenError::UnplannedFrameworkResponder {
                handler: handler.to_string(),
                route: route.to_string(),
            }
        })?;

        responders.insert(
            route,
            FrameworkResponder {
                handler: InjectedDependency {
                    field: binding.field_name.clone(),
                    concrete: handler,
                },
                input,
            },
        );
    }

    Ok(FrameworkResponders { responders })
}

struct HandledRoute {
    handler: CanonicalPath,
    input: FrameworkInput,
    route: CanonicalPath,
}

pub(crate) fn framework_responders(
    endpoints: &ServedEndpoints,
    client_sign_ins: &[ClientSignIn],
    bindings: &ContainerBindings,
) -> Result<FrameworkResponders, CodegenError> {
    planned_responders(bindings, handled_routes(endpoints, client_sign_ins))
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::render_container::render_container;
    use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
    use margaret_http_codegen::framework_input::FrameworkInput;
    use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::HandledRoute;
    use super::planned_responders;
    use crate::codegen_error::CodegenError;

    #[test]
    fn rejects_a_framework_route_whose_handler_the_container_does_not_plan() {
        let index = IndexedSource::new("").index;
        let bindings = render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the empty container is rendered")
        .bindings;

        assert!(matches!(
            planned_responders(
                &bindings,
                vec![HandledRoute {
                    handler: ProviderEndpoint::Token.handler_path(),
                    input: FrameworkInput::Content,
                    route: CanonicalPath::new(vec!["crate".to_string(), "PostToken".to_string()]),
                }],
            ),
            Err(CodegenError::UnplannedFrameworkResponder { handler, route })
                if handler == ProviderEndpoint::Token.handler_path().to_string()
                    && route == "crate::PostToken"
        ));
    }
}
