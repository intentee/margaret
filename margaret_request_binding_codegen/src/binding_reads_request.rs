use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_reads_request(binding: &RequestBinding) -> bool {
    !matches!(
        binding,
        RequestBinding::AssetBag
            | RequestBinding::Forwarder
            | RequestBinding::Next
            | RequestBinding::Routes
            | RequestBinding::Views
    )
}
