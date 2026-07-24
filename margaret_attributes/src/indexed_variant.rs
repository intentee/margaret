use crate::struct_shape::StructShape;

pub struct IndexedVariant {
    identifier: String,
    shape: StructShape,
}

impl IndexedVariant {
    pub(crate) fn new(identifier: String, shape: StructShape) -> Self {
        Self { identifier, shape }
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    pub fn shape(&self) -> StructShape {
        self.shape
    }
}
