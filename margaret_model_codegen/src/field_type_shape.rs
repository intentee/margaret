use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::column_type::ColumnType;

use crate::field_value::FieldValue;

pub(crate) enum FieldTypeShape {
    Column {
        column_type: ColumnType,
        value: FieldValue,
    },
    Key {
        target: CanonicalPath,
    },
}
