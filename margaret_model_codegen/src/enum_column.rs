use quote::quote;
use syn::Type;

use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::struct_shape::StructShape;

use crate::column_type_context::ColumnTypeContext;
use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;

fn validate_enum(
    enum_item: &IndexedItem,
    model: &str,
    column: &str,
) -> Result<(), ModelCodegenError> {
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

    Ok(())
}

pub(crate) fn enum_column(
    context: &mut ColumnTypeContext,
    base: &Type,
    nullable: bool,
    model: &str,
    column: &str,
) -> Result<Option<InferredColumn>, ModelCodegenError> {
    let attribute_index = context.attribute_index;
    let item = context.item;

    let Some(path) = attribute_index.resolve_item_type(item, base) else {
        return Ok(None);
    };

    let Some(enum_item) = attribute_index.item(&path) else {
        return Ok(None);
    };

    if !enum_item.kind().is_enum() {
        return Ok(None);
    }

    if !context.validated_enums.contains(&path) {
        validate_enum(enum_item, model, column)?;
        context.validated_enums.insert(path);
    }

    Ok(Some(InferredColumn {
        column_type: quote!(margaret_model::column_type::ColumnType::Text),
        default: quote!(margaret_model::column_default::ColumnDefault::NotSet),
        nullable,
    }))
}
