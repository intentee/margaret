#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredConsent {
    Implicit,
    Prompted,
}
