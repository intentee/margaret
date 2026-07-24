use margaret_attributes::indexed_field::IndexedField;

pub(crate) struct PositionedField<'field> {
    pub(crate) field: &'field IndexedField,
    pub(crate) position: usize,
}
