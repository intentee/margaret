use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerTransportRequirement {
    Negotiable,
    VerifiedPeerIdentity,
}

impl ServerTransportRequirement {
    #[must_use]
    pub fn for_route(parameters: &[BoundParameter], layers: &[LayerApplication]) -> Self {
        let reads_peer_identity = parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
            || layers.iter().any(|layer| layer.injects_peer_spiffe_id);

        if reads_peer_identity {
            Self::VerifiedPeerIdentity
        } else {
            Self::Negotiable
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_middleware_codegen::layer_application::LayerApplication;
    use margaret_request_binding_codegen::bound_parameter::BoundParameter;
    use margaret_request_binding_codegen::request_binding::RequestBinding;
    use quote::format_ident;

    use super::ServerTransportRequirement;

    fn layer(injects_peer_spiffe_id: bool) -> LayerApplication {
        LayerApplication {
            concrete: CanonicalPath::new(vec!["crate".to_string(), "Guard".to_string()]),
            field: format_ident!("guard"),
            injects_peer_spiffe_id,
            injects_routes: false,
            injects_views: false,
            wrapper: format_ident!("Guard"),
        }
    }

    fn parameter(binding: RequestBinding) -> BoundParameter {
        BoundParameter {
            binding,
            holder: format_ident!("held"),
        }
    }

    #[test]
    fn negotiates_when_nothing_reads_the_peer_identity() {
        assert_eq!(
            ServerTransportRequirement::for_route(
                &[parameter(RequestBinding::CurrentRequest)],
                &[layer(false)]
            ),
            ServerTransportRequirement::Negotiable
        );
    }

    #[test]
    fn requires_a_verified_peer_when_a_parameter_reads_the_peer_identity() {
        assert_eq!(
            ServerTransportRequirement::for_route(
                &[parameter(RequestBinding::PeerSpiffeId)],
                &[layer(false)]
            ),
            ServerTransportRequirement::VerifiedPeerIdentity
        );
    }

    #[test]
    fn requires_a_verified_peer_when_a_layer_reads_the_peer_identity() {
        assert_eq!(
            ServerTransportRequirement::for_route(
                &[parameter(RequestBinding::CurrentRequest)],
                &[layer(true)]
            ),
            ServerTransportRequirement::VerifiedPeerIdentity
        );
    }
}
