#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoutePosition {
    index: usize,
}

impl RoutePosition {
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}
