use crate::field_value::FieldValue;
use crate::resolved_column::ResolvedColumn;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelField {
    pub columns: Vec<ResolvedColumn>,
    pub name: String,
    pub nullable: bool,
    pub start: usize,
    pub value: FieldValue,
}
