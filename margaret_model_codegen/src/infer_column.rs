use quote::ToTokens;
use quote::quote;

use crate::column_type_context::ColumnTypeContext;
use crate::column_type_source::ColumnTypeSource;
use crate::declared_column_type::DeclaredColumnType;
use crate::enum_column::enum_column;
use crate::infer_column_type::infer_column_type;
use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;
use crate::numeric_digits::NumericDigits;

fn numeric_column(precision: u32, scale: u32, nullable: bool) -> InferredColumn {
    InferredColumn {
        column_type: quote!(
            margaret::framework::model::column_type::ColumnType::Numeric {
                precision: #precision,
                scale: #scale,
            }
        ),
        default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
        nullable,
    }
}

pub(crate) fn infer_column(
    ColumnTypeContext {
        attribute_index,
        item,
        validated_enums,
    }: &mut ColumnTypeContext,
    declared_column_type: &DeclaredColumnType,
    numeric_digits: &NumericDigits,
    model: &str,
    column: &str,
) -> Result<InferredColumn, ModelCodegenError> {
    let DeclaredColumnType {
        base,
        declared,
        nullable,
    } = declared_column_type;

    match (
        ColumnTypeSource::of(attribute_index, item, base),
        numeric_digits,
    ) {
        (ColumnTypeSource::Decimal, NumericDigits::Declared { precision, scale }) => {
            Ok(numeric_column(*precision, *scale, *nullable))
        }
        (ColumnTypeSource::Decimal, NumericDigits::NotDeclared) => {
            Err(ModelCodegenError::NumericColumnRequiresDigits {
                column: column.to_string(),
                model: model.to_string(),
            })
        }
        (_, NumericDigits::Declared { .. }) => {
            Err(ModelCodegenError::NumericDigitsOnNonNumericColumn {
                column: column.to_string(),
                model: model.to_string(),
                rust_type: declared.to_token_stream().to_string(),
            })
        }
        (ColumnTypeSource::LocalEnum(enum_item), NumericDigits::NotDeclared) => {
            enum_column(validated_enums, enum_item, *nullable, model, column)
        }
        (ColumnTypeSource::LocalItem(local_item), NumericDigits::NotDeclared) => {
            Err(ModelCodegenError::LocalColumnTypeIsNotAnEnum {
                column: column.to_string(),
                model: model.to_string(),
                resolved_type: local_item.canonical_path().to_string(),
                rust_type: declared.to_token_stream().to_string(),
            })
        }
        (ColumnTypeSource::DeclaredType, NumericDigits::NotDeclared) => {
            infer_column_type(declared_column_type, model, column)
        }
    }
}
