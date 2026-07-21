use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_shadows_request(binding: &RequestBinding) -> bool {
    matches!(
        binding,
        RequestBinding::Raw { .. }
            | RequestBinding::Bound { .. }
            | RequestBinding::FormRequest { .. }
            | RequestBinding::PeerSpiffeId
    )
}
