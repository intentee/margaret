#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResourceTarget {
    Named(&'static str),
    Unnamed,
}
