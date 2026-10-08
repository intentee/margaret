#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MessageCardinality {
    Single,
    Stream,
}
