pub(crate) struct AttributeLocation {
    item: usize,
    attribute: usize,
}

impl AttributeLocation {
    pub(crate) fn new(item: usize, attribute: usize) -> Self {
        Self { item, attribute }
    }

    pub(crate) fn attribute(&self) -> usize {
        self.attribute
    }

    pub(crate) fn item(&self) -> usize {
        self.item
    }
}
