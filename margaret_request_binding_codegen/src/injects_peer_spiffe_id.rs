use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn injects_peer_spiffe_id(parameters: &[BoundParameter]) -> bool {
    parameters
        .iter()
        .any(|parameter| matches!(parameter.binding, RequestBinding::PeerSpiffeId))
}
