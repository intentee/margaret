#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathSegment<'path> {
    Text(&'path str),
    TrailingSeparator,
}
