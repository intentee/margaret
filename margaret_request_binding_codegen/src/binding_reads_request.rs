use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_reads_request(binding: &RequestBinding) -> bool {
    !matches!(
        binding,
        RequestBinding::AssetBag
            | RequestBinding::Forwarder
            | RequestBinding::Injectable { .. }
            | RequestBinding::Next
            | RequestBinding::RequestBodyStream
            | RequestBinding::Routes
            | RequestBinding::Views
    )
}

#[cfg(test)]
mod tests {
    use super::binding_reads_request;
    use crate::request_binding::RequestBinding;

    #[test]
    fn leaves_the_request_unread_for_a_streamed_body() {
        assert!(!binding_reads_request(&RequestBinding::RequestBodyStream));
    }
}
