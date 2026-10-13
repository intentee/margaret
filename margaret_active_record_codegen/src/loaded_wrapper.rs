#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoadedWrapper {
    Bare,
    Children,
    Optional,
}
