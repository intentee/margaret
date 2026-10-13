use margaret_model::column_default::ColumnDefault;
use margaret_model_codegen::model_field::ModelField;

pub(crate) fn is_defaulted(field: &ModelField) -> bool {
    field
        .columns
        .iter()
        .all(|column| column.default != ColumnDefault::NotSet)
}
