use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_roots(binding: &RequestBinding) -> Vec<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => match &application.challenge {
            AuthenticatedUserChallenge::Bearer { issuer_client } => {
                vec![&application.concrete, &issuer_client.concrete]
            }
            AuthenticatedUserChallenge::Unchallenged => vec![&application.concrete],
        },
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => vec![binder_provider],
        RequestBinding::Injectable { dependency } => vec![&dependency.concrete],
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::BearerToken { .. }
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_container::injected_dependency::InjectedDependency;

    use super::binding_roots;
    use crate::authenticated_user_application::AuthenticatedUserApplication;
    use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
    use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
    use crate::request_binding::RequestBinding;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    #[test]
    fn identifies_a_route_parameter_binder_as_a_retained_root() {
        let binder_provider =
            CanonicalPath::new(vec!["crate".to_string(), "UserBinder".to_string()]);
        let binding = RequestBinding::BoundRouteParameter {
            binder_field: "user_binder".to_string(),
            binder_provider: binder_provider.clone(),
            path_key: "user".to_string(),
        };

        assert_eq!(binding_roots(&binding), vec![&binder_provider]);
    }

    #[test]
    fn identifies_an_injected_dependency_as_a_retained_root() {
        let concrete = CanonicalPath::new(vec!["crate".to_string(), "Store".to_string()]);
        let binding = RequestBinding::Injectable {
            dependency: InjectedDependency {
                concrete: concrete.clone(),
                field: "store".to_string(),
            },
        };

        assert_eq!(binding_roots(&binding), vec![&concrete]);
    }

    #[test]
    fn retains_the_token_issuer_client_an_authenticated_user_provider_verifies_with() {
        let binding = RequestBinding::AuthenticatedUser {
            application: AuthenticatedUserApplication {
                challenge: AuthenticatedUserChallenge::Bearer {
                    issuer_client: InjectedDependency {
                        concrete: path("Client"),
                        field: "client".to_string(),
                    },
                },
                concrete: path("RunnerProvider"),
                field: "runner_provider".to_string(),
                injects_peer_spiffe_id: false,
                injects_routes: false,
                injects_views: false,
                model: path("Runner"),
                wrapper: format_ident!("RunnerProvider"),
            },
            requirement: AuthenticatedUserRequirement::Required,
        };

        assert_eq!(
            binding_roots(&binding),
            vec![&path("RunnerProvider"), &path("Client")]
        );
    }
}
