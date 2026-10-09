use syn::ext::IdentExt;

use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_name::is_snake_case_name;
use margaret_attributes::select_framework_attributes::select_framework_attributes;
use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::on_delete::OnDelete;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;

use crate::classified_field_type::ClassifiedFieldType;
use crate::classify_field_type::classify_field_type;
use crate::collected_column::CollectedColumn;
use crate::collected_field::CollectedField;
use crate::collected_field_shape::CollectedFieldShape;
use crate::column_arguments::ColumnArguments;
use crate::column_default_argument::ColumnDefaultArgument;
use crate::declared_field_index::declared_field_index;
use crate::enum_variants::EnumVariants;
use crate::field_index::FieldIndex;
use crate::field_type_shape::FieldTypeShape;
use crate::foreign_key_arguments::ForeignKeyArguments;
use crate::model_codegen_error::ModelCodegenError;

fn field_names(field: &IndexedField, model: &str) -> Result<FieldNames, ModelCodegenError> {
    match field.identifier() {
        FieldIdentifier::Named(identifier) => {
            let column_base = identifier.unraw().to_string();

            if is_snake_case_name(&column_base) {
                Ok(FieldNames {
                    column_base,
                    name: identifier.to_string(),
                })
            } else {
                Err(ModelCodegenError::FieldNameNotSnakeCase {
                    field: identifier.to_string(),
                    model: model.to_string(),
                })
            }
        }
        FieldIdentifier::Positional(position) => Err(ModelCodegenError::ModelRequiresNamedFields {
            model: model.to_string(),
            position: *position,
        }),
    }
}

fn reject_model_attributes(field: &IndexedField, model: &str) -> Result<(), ModelCodegenError> {
    if !select_framework_attributes(field.attributes(), FrameworkAttribute::PrimaryKey).is_empty() {
        return Err(ModelCodegenError::PrimaryKeyIsNotAFieldAttribute {
            field: field.identifier().to_string(),
            model: model.to_string(),
        });
    }

    if !select_framework_attributes(field.attributes(), FrameworkAttribute::Unique).is_empty() {
        return Err(ModelCodegenError::UniqueIsNotAFieldAttribute {
            field: field.identifier().to_string(),
            model: model.to_string(),
        });
    }

    Ok(())
}

fn field_index(
    field: &IndexedField,
    name: &str,
    model: &str,
) -> Result<FieldIndex, ModelCodegenError> {
    match field.framework_attribute(FrameworkAttribute::Index) {
        None => Ok(FieldIndex::Absent),
        Some(declared) => declared_field_index(declared.args()?, model, name),
    }
}

fn column_name(
    declared: Option<String>,
    column_base: &str,
    model: &str,
) -> Result<String, ModelCodegenError> {
    let name = declared.unwrap_or_else(|| column_base.to_string());

    if !is_snake_case_name(&name) {
        return Err(ModelCodegenError::InvalidColumnName {
            column: name,
            model: model.to_string(),
        });
    }

    validate_identifier_length(&name).map_err(|source| ModelCodegenError::ColumnNameTooLong {
        model: model.to_string(),
        source,
    })?;

    Ok(name)
}

fn key_on_delete(
    index: &AttributeIndex,
    item: &IndexedItem,
    field: &IndexedField,
    name: &str,
    nullable: bool,
    model: &str,
) -> Result<OnDelete, ModelCodegenError> {
    let on_delete = match field.framework_attribute(FrameworkAttribute::ForeignKey) {
        None => OnDelete::NoAction,
        Some(declared) => {
            ForeignKeyArguments::parse(declared.args()?, index, item, model, name)?.on_delete
        }
    };

    if on_delete == OnDelete::SetNull && !nullable {
        return Err(ModelCodegenError::SetNullRequiresNullableKey {
            field: name.to_string(),
            model: model.to_string(),
        });
    }

    Ok(on_delete)
}

fn column_default(
    declared: ColumnDefaultArgument,
    column_type: ColumnType,
    field: &str,
    model: &str,
) -> Result<ColumnDefault, ModelCodegenError> {
    match declared {
        ColumnDefaultArgument::Missing => Ok(ColumnDefault::NotSet),
        ColumnDefaultArgument::Known(column_default) if column_type == ColumnType::Uuid => {
            Ok(column_default)
        }
        ColumnDefaultArgument::Known(_) => {
            Err(ModelCodegenError::ColumnDefaultRequiresUuidColumn {
                field: field.to_string(),
                model: model.to_string(),
            })
        }
        ColumnDefaultArgument::Unknown(path) => Err(ModelCodegenError::UnknownColumnDefault {
            default: format_path(&path),
            field: field.to_string(),
            model: model.to_string(),
        }),
    }
}

struct FieldNames {
    column_base: String,
    name: String,
}

pub(crate) fn collect_field(
    index: &AttributeIndex,
    item: &IndexedItem,
    field: &IndexedField,
    enum_variants: &mut EnumVariants,
    table: &str,
    model: &str,
) -> Result<CollectedField, ModelCodegenError> {
    reject_model_attributes(field, model)?;

    let FieldNames { column_base, name } = field_names(field, model)?;
    let column_attribute = field
        .framework_attribute(FrameworkAttribute::Column)
        .ok_or_else(|| ModelCodegenError::UnattributedField {
            field: name.clone(),
            model: model.to_string(),
        })?;
    let ColumnArguments {
        check,
        default,
        name: declared_column,
        numeric_digits,
        primary_key,
        unique,
    } = ColumnArguments::parse(column_attribute.args()?, model, &name)?;
    let declared_default = ColumnDefaultArgument::of(index, item, default);
    let index_declaration = field_index(field, &name, model)?;
    let ClassifiedFieldType { nullable, shape } = classify_field_type(
        index,
        item,
        field.ty(),
        &numeric_digits,
        enum_variants,
        model,
        &name,
    )?;
    let shape = match shape {
        FieldTypeShape::Column { column_type, value } => {
            if field
                .framework_attribute(FrameworkAttribute::ForeignKey)
                .is_some()
            {
                return Err(ModelCodegenError::ForeignKeyRequiresKeyField {
                    field: name,
                    model: model.to_string(),
                });
            }

            let column = column_name(declared_column, &column_base, model)?;

            CollectedFieldShape::Column(CollectedColumn {
                checks: check.resolve(column_type, table, &column, model)?,
                column_type,
                default: column_default(declared_default, column_type, &name, model)?,
                name: column,
                value,
            })
        }
        FieldTypeShape::Key { target } => {
            if declared_column.is_some() {
                return Err(ModelCodegenError::ForeignKeyColumnNameIsDerived {
                    field: name,
                    model: model.to_string(),
                });
            }

            if check.is_declared() {
                return Err(ModelCodegenError::ForeignKeyCannotDeclareCheckConstraint {
                    field: name,
                    model: model.to_string(),
                });
            }

            if !matches!(declared_default, ColumnDefaultArgument::Missing) {
                return Err(ModelCodegenError::ForeignKeyCannotDeclareDefault {
                    field: name,
                    model: model.to_string(),
                });
            }

            CollectedFieldShape::Key {
                column_base,
                on_delete: key_on_delete(index, item, field, &name, nullable, model)?,
                target,
            }
        }
    };

    Ok(CollectedField {
        index: index_declaration,
        name,
        nullable,
        primary_key,
        shape,
        unique,
    })
}
