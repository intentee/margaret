#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KeyTargeting {
    Referenced,
    Unreferenced,
}
