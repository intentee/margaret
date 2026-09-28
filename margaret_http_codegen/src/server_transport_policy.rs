use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::injects_peer_spiffe_id::injects_peer_spiffe_id;
use margaret_request_binding_codegen::request_binding::RequestBinding;

fn infers_user_from_peer_spiffe_id(parameters: &[BoundParameter]) -> bool {
    parameters.iter().any(|parameter| {
        matches!(
            &parameter.binding,
            RequestBinding::AuthenticatedUser { application, .. }
                if application.injects_peer_spiffe_id
        )
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerTransportPolicy {
    Negotiable,
    PinnedSpiffeMtls,
}

impl ServerTransportPolicy {
    #[must_use]
    pub fn required_by(parameters: &[BoundParameter], layers: &[LayerApplication]) -> Self {
        if injects_peer_spiffe_id(parameters)
            || infers_user_from_peer_spiffe_id(parameters)
            || layers.iter().any(|layer| layer.injects_peer_spiffe_id)
        {
            Self::PinnedSpiffeMtls
        } else {
            Self::Negotiable
        }
    }

    #[must_use]
    pub fn combined_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Negotiable, Self::Negotiable) => Self::Negotiable,
            (Self::PinnedSpiffeMtls, _) | (_, Self::PinnedSpiffeMtls) => Self::PinnedSpiffeMtls,
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_middleware_codegen::layer_application::LayerApplication;
    use margaret_request_binding_codegen::authenticated_user_application::AuthenticatedUserApplication;
    use margaret_request_binding_codegen::authenticated_user_challenge::AuthenticatedUserChallenge;
    use margaret_request_binding_codegen::authenticated_user_requirement::AuthenticatedUserRequirement;
    use margaret_request_binding_codegen::bound_parameter::BoundParameter;
    use margaret_request_binding_codegen::request_binding::RequestBinding;

    use super::ServerTransportPolicy;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn parameter(binding: RequestBinding) -> BoundParameter {
        BoundParameter {
            binding,
            holder: format_ident!("held"),
        }
    }

    fn authenticated_user(injects_peer_spiffe_id: bool) -> BoundParameter {
        parameter(RequestBinding::AuthenticatedUser {
            application: AuthenticatedUserApplication {
                challenge: AuthenticatedUserChallenge::Unchallenged,
                concrete: path("PeerUserProvider"),
                field: "peer_user_provider".to_string(),
                injects_peer_spiffe_id,
                injects_routes: false,
                injects_views: false,
                model: path("User"),
                oidc_token_verifiers: Vec::new(),
                wrapper: format_ident!("PeerUserProvider"),
            },
            requirement: AuthenticatedUserRequirement::Required,
        })
    }

    fn layer(injects_peer_spiffe_id: bool) -> LayerApplication {
        LayerApplication {
            concrete: path("Guard"),
            field: format_ident!("guard"),
            injects_peer_spiffe_id,
            injects_routes: false,
            injects_views: false,
            wrapper: format_ident!("Guard"),
        }
    }

    #[test]
    fn pins_a_site_that_injects_the_peer_spiffe_id() {
        assert_eq!(
            ServerTransportPolicy::required_by(&[parameter(RequestBinding::PeerSpiffeId)], &[]),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
    }

    #[test]
    fn pins_a_site_whose_authenticated_user_is_inferred_from_the_peer_spiffe_id() {
        assert_eq!(
            ServerTransportPolicy::required_by(&[authenticated_user(true)], &[]),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
    }

    #[test]
    fn pins_a_site_whose_middleware_reads_the_peer_spiffe_id() {
        assert_eq!(
            ServerTransportPolicy::required_by(&[], &[layer(true)]),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
    }

    #[test]
    fn negotiates_a_site_that_never_reads_the_peer_spiffe_id() {
        assert_eq!(
            ServerTransportPolicy::required_by(
                &[
                    parameter(RequestBinding::CurrentRequest),
                    authenticated_user(false)
                ],
                &[layer(false)]
            ),
            ServerTransportPolicy::Negotiable
        );
    }

    #[test]
    fn stays_negotiable_only_when_both_sides_are_negotiable() {
        assert_eq!(
            ServerTransportPolicy::Negotiable.combined_with(ServerTransportPolicy::Negotiable),
            ServerTransportPolicy::Negotiable
        );
        assert_eq!(
            ServerTransportPolicy::Negotiable
                .combined_with(ServerTransportPolicy::PinnedSpiffeMtls),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
        assert_eq!(
            ServerTransportPolicy::PinnedSpiffeMtls
                .combined_with(ServerTransportPolicy::Negotiable),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
    }
}
