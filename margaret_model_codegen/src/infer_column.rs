use quote::ToTokens;

use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;

use crate::column_type_context::ColumnTypeContext;
use crate::column_type_source::ColumnTypeSource;
use crate::declared_column_type::DeclaredColumnType;
use crate::enum_column::enum_column;
use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;
use crate::numeric_digits::NumericDigits;

fn numeric_column(precision: u32, scale: u32, nullable: bool) -> InferredColumn {
    InferredColumn {
        column_type: ColumnType::Numeric { precision, scale },
        default: ColumnDefault::NotSet,
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

    let source = ColumnTypeSource::of(attribute_index, item, base);

    match numeric_digits {
        NumericDigits::Declared { precision, scale } => match source {
            ColumnTypeSource::Decimal => Ok(numeric_column(*precision, *scale, *nullable)),
            ColumnTypeSource::Known(_)
            | ColumnTypeSource::LocalEnum(_)
            | ColumnTypeSource::LocalItem(_)
            | ColumnTypeSource::Uninferrable => {
                Err(ModelCodegenError::NumericDigitsOnNonNumericColumn {
                    column: column.to_string(),
                    model: model.to_string(),
                    rust_type: declared.to_token_stream().to_string(),
                })
            }
        },
        NumericDigits::NotDeclared => match source {
            ColumnTypeSource::Decimal => Err(ModelCodegenError::NumericColumnRequiresDigits {
                column: column.to_string(),
                model: model.to_string(),
            }),
            ColumnTypeSource::LocalEnum(enum_item) => {
                enum_column(validated_enums, enum_item, *nullable, model, column)
            }
            ColumnTypeSource::LocalItem(local_item) => {
                Err(ModelCodegenError::LocalColumnTypeIsNotAnEnum {
                    column: column.to_string(),
                    model: model.to_string(),
                    resolved_type: local_item.canonical_path().to_string(),
                    rust_type: declared.to_token_stream().to_string(),
                })
            }
            ColumnTypeSource::Known(known) => Ok(InferredColumn {
                nullable: *nullable,
                ..known
            }),
            ColumnTypeSource::Uninferrable => Err(ModelCodegenError::UninferrableColumnType {
                column: column.to_string(),
                model: model.to_string(),
                rust_type: declared.to_token_stream().to_string(),
            }),
        },
    }
}
