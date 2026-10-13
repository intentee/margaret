#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DeclaredClientKeySource {
    Own,
    Published,
}
