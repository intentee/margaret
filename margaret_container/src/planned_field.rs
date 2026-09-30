use crate::provided_type::ProvidedType;

pub(crate) struct PlannedField {
    pub(crate) field_name: String,
    pub(crate) provided: ProvidedType,
}
