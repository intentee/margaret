use std::collections::HashSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::struct_shape::StructShape;
use margaret_model::column_type::ColumnType;

use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;

fn text_column(nullable: bool) -> InferredColumn {
    InferredColumn {
        column_type: ColumnType::Text,
        nullable,
    }
}

pub(crate) fn enum_column(
    validated_enums: &mut HashSet<CanonicalPath>,
    enum_item: &IndexedItem,
    nullable: bool,
    model: &str,
    column: &str,
) -> Result<InferredColumn, ModelCodegenError> {
    if validated_enums.contains(enum_item.canonical_path()) {
        return Ok(text_column(nullable));
    }

    if enum_item.variants().is_empty() {
        return Err(ModelCodegenError::EmptyEnumColumn {
            column: column.to_string(),
            enum_type: enum_item.canonical_path().to_string(),
            model: model.to_string(),
        });
    }

    for variant in enum_item.variants() {
        if !matches!(variant.shape(), StructShape::Unit) {
            return Err(ModelCodegenError::EnumColumnVariantNotUnit {
                column: column.to_string(),
                enum_type: enum_item.canonical_path().to_string(),
                model: model.to_string(),
                variant: variant.identifier().to_string(),
            });
        }
    }

    validated_enums.insert(enum_item.canonical_path().clone());

    Ok(text_column(nullable))
}
