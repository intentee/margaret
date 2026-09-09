use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn injects_forwarder(parameters: &[BoundParameter]) -> bool {
    parameters
        .iter()
        .any(|parameter| matches!(parameter.binding, RequestBinding::Forwarder))
}
