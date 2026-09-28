use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_shadows_request(binding: &RequestBinding) -> bool {
    matches!(
        binding,
        RequestBinding::AssetBag
            | RequestBinding::AuthenticatedUser { .. }
            | RequestBinding::RouteParameterValue { .. }
            | RequestBinding::BoundRouteParameter { .. }
            | RequestBinding::FormRequest { .. }
            | RequestBinding::OidcToken { .. }
            | RequestBinding::PeerSpiffeId
    )
}
