#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ColumnId {
    index: usize,
}

impl ColumnId {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }
}
