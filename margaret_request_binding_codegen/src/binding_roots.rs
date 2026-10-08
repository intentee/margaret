use margaret_attributes::canonical_path::CanonicalPath;

use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_roots(binding: &RequestBinding) -> Vec<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => {
            let mut roots = vec![&application.concrete];

            roots.extend(
                application
                    .challenge
                    .dependencies()
                    .into_iter()
                    .map(|dependency| &dependency.concrete),
            );

            roots
        }
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => vec![binder_provider],
        RequestBinding::Injectable { dependency } => vec![&dependency.concrete],
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormContent { .. }
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::JsonContent { .. }
        | RequestBinding::Next
        | RequestBinding::BearerToken { .. }
        | RequestBinding::IntrospectedBearerToken { .. }
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RequestBodyStream
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::UploadedFiles
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
    use crate::route_parameter_lookup::RouteParameterLookup;

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
            lookup: RouteParameterLookup::Binder,
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
    fn retains_every_trusted_issuer_an_authenticated_user_provider_admits_tokens_of() {
        let binding = RequestBinding::AuthenticatedUser {
            application: AuthenticatedUserApplication {
                challenge: AuthenticatedUserChallenge::Bearer {
                    trusted_issuers: vec![
                        InjectedDependency {
                            concrete: path("Partner"),
                            field: "partner".to_string(),
                        },
                        InjectedDependency {
                            concrete: path("Upstream"),
                            field: "upstream".to_string(),
                        },
                    ],
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
            vec![&path("RunnerProvider"), &path("Partner"), &path("Upstream")]
        );
    }
}
