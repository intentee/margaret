use margaret_http_codegen::framework_input::FrameworkInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EndpointAdmission {
    Admitted(FrameworkInput),
    Refused,
}
