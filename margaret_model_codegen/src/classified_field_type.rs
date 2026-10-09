use crate::field_type_shape::FieldTypeShape;

pub(crate) struct ClassifiedFieldType {
    pub(crate) nullable: bool,
    pub(crate) shape: FieldTypeShape,
}
