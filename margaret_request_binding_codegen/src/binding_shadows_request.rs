use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_shadows_request(binding: &RequestBinding) -> bool {
    matches!(
        binding,
        RequestBinding::AssetBag
            | RequestBinding::AuthenticatedUser { .. }
            | RequestBinding::RouteParameterValue { .. }
            | RequestBinding::BoundRouteParameter { .. }
            | RequestBinding::FormContent { .. }
            | RequestBinding::FormRequest { .. }
            | RequestBinding::JsonContent { .. }
            | RequestBinding::RequestBodyStream
            | RequestBinding::UploadedFiles
            | RequestBinding::BearerToken { .. }
            | RequestBinding::IntrospectedBearerToken { .. }
            | RequestBinding::PeerSpiffeId
    )
}
