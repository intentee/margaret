#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SignInEndpointVariant {
    Callback,
    Start,
}
