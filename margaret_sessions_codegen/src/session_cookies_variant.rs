#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionCookiesVariant {
    HostOnly,
    SharedWithDomain,
}
