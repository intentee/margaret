use quote::ToTokens;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::item_kind::ItemKind;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_model::column_type::ColumnType;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;

use crate::classified_field_type::ClassifiedFieldType;
use crate::date_time_canonical_path::DATE_TIME_CANONICAL_PATH;
use crate::decimal_canonical_path::DECIMAL_CANONICAL_PATH;
use crate::enum_variants::EnumVariants;
use crate::field_type_shape::FieldTypeShape;
use crate::field_value::FieldValue;
use crate::json_canonical_path::JSON_CANONICAL_PATH;
use crate::key_canonical_path::KEY_CANONICAL_PATH;
use crate::model_codegen_error::ModelCodegenError;
use crate::numeric_digits::NumericDigits;
use crate::resolve_rust_type::resolve_rust_type;
use crate::resolved_rust_type::ResolvedRustType;
use crate::secret_text_canonical_path::SECRET_TEXT_CANONICAL_PATH;
use crate::utc_canonical_path::UTC_CANONICAL_PATH;
use crate::uuid_canonical_path::UUID_CANONICAL_PATH;

fn scalar(column_type: ColumnType, rust_type: ResolvedRustType) -> FieldTypeShape {
    FieldTypeShape::Column {
        column_type,
        value: FieldValue::Scalar { rust_type },
    }
}

fn primitive_column_type(primitive: &str) -> Option<ColumnType> {
    match primitive {
        "bool" => Some(ColumnType::Boolean),
        "f32" => Some(ColumnType::Real),
        "f64" => Some(ColumnType::DoublePrecision),
        "i32" => Some(ColumnType::Integer),
        "i64" => Some(ColumnType::BigInt),
        _ => None,
    }
}

fn single_argument(
    resolved: &ResolvedRustType,
    model: &str,
    field: &str,
) -> Result<ResolvedRustType, ModelCodegenError> {
    match resolved.arguments.as_slice() {
        [argument] => Ok(argument.clone()),
        _ => Err(ModelCodegenError::WrapperRequiresOneArgument {
            field: field.to_string(),
            model: model.to_string(),
            wrapper: resolved.path.to_string(),
        }),
    }
}

fn is_byte_vector(resolved: &ResolvedRustType) -> bool {
    StandardLibraryItem::from_canonical(&resolved.path) == Some(StandardLibraryItem::Vec)
        && matches!(
            resolved.arguments.as_slice(),
            [element] if element.path.segments() == ["u8"] && element.arguments.is_empty()
        )
}

fn local_shape(
    index: &AttributeIndex,
    local: &IndexedItem,
    enum_variants: &mut EnumVariants,
    model: &str,
    field: &str,
    rust_type: &str,
) -> Result<FieldTypeShape, ModelCodegenError> {
    match local.kind() {
        ItemKind::Enum => Ok(FieldTypeShape::Column {
            column_type: ColumnType::Text,
            value: FieldValue::Enum {
                path: local.canonical_path().clone(),
                variants: enum_variants.of(local, model, field)?,
            },
        }),
        _ if index
            .item(local.canonical_path())
            .is_some_and(|item| item.has_framework_attribute(FrameworkAttribute::Model)) =>
        {
            Err(ModelCodegenError::ModelFieldRequiresKey {
                field: field.to_string(),
                model: model.to_string(),
                target: local.canonical_path().to_string(),
            })
        }
        _ => Err(ModelCodegenError::LocalColumnTypeIsNotAnEnum {
            field: field.to_string(),
            model: model.to_string(),
            resolved_type: local.canonical_path().to_string(),
            rust_type: rust_type.to_string(),
        }),
    }
}

fn shape_of(
    index: &AttributeIndex,
    item: &IndexedItem,
    base: &Type,
    numeric_digits: &NumericDigits,
    enum_variants: &mut EnumVariants,
    model: &str,
    field: &str,
) -> Result<FieldTypeShape, ModelCodegenError> {
    let resolved = resolve_rust_type(index, item, base, model, field)?;
    let rust_type = base.to_token_stream().to_string();

    if resolved.path == *DECIMAL_CANONICAL_PATH {
        return match numeric_digits {
            NumericDigits::Declared { precision, scale } => Ok(scalar(
                ColumnType::Numeric {
                    precision: *precision,
                    scale: *scale,
                },
                resolved,
            )),
            NumericDigits::NotDeclared => Err(ModelCodegenError::NumericColumnRequiresDigits {
                field: field.to_string(),
                model: model.to_string(),
            }),
        };
    }

    if let NumericDigits::Declared { .. } = numeric_digits {
        return Err(ModelCodegenError::NumericDigitsOnNonNumericColumn {
            field: field.to_string(),
            model: model.to_string(),
            rust_type,
        });
    }

    if resolved.path == *KEY_CANONICAL_PATH {
        return Ok(FieldTypeShape::Key {
            target: single_argument(&resolved, model, field)?.path,
        });
    }

    if resolved.path == *JSON_CANONICAL_PATH {
        return Ok(FieldTypeShape::Column {
            column_type: ColumnType::Text,
            value: FieldValue::Json {
                payload: single_argument(&resolved, model, field)?,
            },
        });
    }

    if resolved.path == *DATE_TIME_CANONICAL_PATH {
        return if single_argument(&resolved, model, field)?.path == *UTC_CANONICAL_PATH {
            Ok(scalar(ColumnType::Timestamptz, resolved))
        } else {
            Err(ModelCodegenError::TimestamptzRequiresUtc {
                field: field.to_string(),
                model: model.to_string(),
                rust_type,
            })
        };
    }

    if resolved.path == *UUID_CANONICAL_PATH {
        return Ok(scalar(ColumnType::Uuid, resolved));
    }

    if resolved.path == *SECRET_TEXT_CANONICAL_PATH {
        return Ok(scalar(ColumnType::Text, resolved));
    }

    if StandardLibraryItem::from_canonical(&resolved.path) == Some(StandardLibraryItem::String) {
        return Ok(scalar(ColumnType::Text, resolved));
    }

    if is_byte_vector(&resolved) {
        return Ok(scalar(ColumnType::Bytea, resolved));
    }

    if let Some(local) = index.item(&resolved.path) {
        return local_shape(index, local, enum_variants, model, field, &rust_type);
    }

    match resolved.path.segments() {
        [primitive] if resolved.arguments.is_empty() => primitive_column_type(primitive)
            .map(|column_type| scalar(column_type, resolved.clone()))
            .ok_or_else(|| ModelCodegenError::UninferrableColumnType {
                field: field.to_string(),
                model: model.to_string(),
                rust_type: rust_type.clone(),
            }),
        _ => Err(ModelCodegenError::UninferrableColumnType {
            field: field.to_string(),
            model: model.to_string(),
            rust_type,
        }),
    }
}

pub(crate) fn classify_field_type(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    numeric_digits: &NumericDigits,
    enum_variants: &mut EnumVariants,
    model: &str,
    field: &str,
) -> Result<ClassifiedFieldType, ModelCodegenError> {
    let wrapped = peel_standard_wrapper(index, item, declared, &[StandardLibraryItem::Option]);

    Ok(ClassifiedFieldType {
        nullable: wrapped.is_some(),
        shape: shape_of(
            index,
            item,
            wrapped.unwrap_or(declared),
            numeric_digits,
            enum_variants,
            model,
            field,
        )?,
    })
}
