use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_unique_attribute::select_unique_attribute;

use crate::collected_model::CollectedModel;
use crate::column_arguments::ColumnArguments;
use crate::deferred_foreign_key::DeferredForeignKey;
use crate::foreign_key_column_name::foreign_key_column_name;
use crate::foreign_key_target::ForeignKeyTarget;
use crate::foreign_key_target_column::ForeignKeyTargetColumn;
use crate::infer_column_type::infer_column_type;
use crate::inferred_column::InferredColumn;
use crate::model::Model;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::option_inner::option_inner;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;

const POSTGRES_MAX_IDENTIFIER_BYTES: usize = 63;

fn field_display(identifier: &FieldIdentifier) -> String {
    match identifier {
        FieldIdentifier::Named(field_name) => field_name.clone(),
        FieldIdentifier::Positional(position) => position.to_string(),
    }
}

fn resolve_column_name(
    name: Option<String>,
    identifier: &FieldIdentifier,
    model: &str,
) -> Result<String, ModelCodegenError> {
    match name {
        Some(explicit) => Ok(explicit),
        None => match identifier {
            FieldIdentifier::Named(field_name) => Ok(field_name.clone()),
            FieldIdentifier::Positional(position) => {
                Err(ModelCodegenError::PositionalColumnRequiresName {
                    model: model.to_string(),
                    position: *position,
                })
            }
        },
    }
}

fn register_column_name(
    column_name: String,
    model: &str,
    seen_columns: &mut HashSet<String>,
) -> Result<String, ModelCodegenError> {
    if !is_snake_case_identifier(&column_name) {
        return Err(ModelCodegenError::InvalidColumnName {
            column: column_name,
            model: model.to_string(),
        });
    }

    let length = column_name.len();

    if length > POSTGRES_MAX_IDENTIFIER_BYTES {
        return Err(ModelCodegenError::ColumnNameTooLong {
            column: column_name,
            length,
            model: model.to_string(),
        });
    }

    if !seen_columns.insert(column_name.clone()) {
        return Err(ModelCodegenError::DuplicateColumnName {
            column: column_name,
            model: model.to_string(),
        });
    }

    Ok(column_name)
}

fn resolve_scalar_column(
    ColumnArguments { name, primary_key }: ColumnArguments,
    field: &IndexedField,
    model: &str,
    seen_columns: &mut HashSet<String>,
) -> Result<ResolvedColumn, ModelCodegenError> {
    let column_name = resolve_column_name(name, field.identifier(), model)?;
    let column_name = register_column_name(column_name, model, seen_columns)?;
    let inferred = infer_column_type(field.ty(), model, &column_name)?;

    Ok(ResolvedColumn {
        inferred,
        name: column_name,
        primary_key,
    })
}

fn defer_foreign_key(
    ColumnArguments { name, primary_key }: ColumnArguments,
    field: &IndexedField,
    model: &str,
    index: &AttributeIndex,
    item: &IndexedItem,
) -> Result<DeferredForeignKey, ModelCodegenError> {
    if name.is_some() {
        return Err(ModelCodegenError::ForeignKeyColumnNameIsDerived {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    if primary_key {
        return Err(ModelCodegenError::ForeignKeyCannotBePrimaryKey {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    let field_name = match field.identifier() {
        FieldIdentifier::Named(field_name) => field_name.clone(),
        FieldIdentifier::Positional(position) => {
            return Err(ModelCodegenError::ForeignKeyRequiresNamedField {
                model: model.to_string(),
                position: *position,
            });
        }
    };

    let (target_type, nullable) = match option_inner(field.ty()) {
        Some(inner) => (inner, true),
        None => (field.ty(), false),
    };

    let rust_type = field.ty().to_token_stream().to_string();

    let target_path = index.resolve_item_type(item, target_type).ok_or_else(|| {
        ModelCodegenError::ForeignKeyTargetNotAModel {
            field: field_name.clone(),
            model: model.to_string(),
            rust_type: rust_type.clone(),
        }
    })?;

    Ok(DeferredForeignKey {
        field_name,
        nullable,
        rust_type,
        target_path,
    })
}

fn collect_models(
    index: &AttributeIndex,
    targets: &mut HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Vec<CollectedModel>, ModelCodegenError> {
    let selector = AttributeSelector::from_marker("model");
    let column_selector = AttributeSelector::from_marker("column");
    let foreign_key_selector = AttributeSelector::from_marker("foreign_key");
    let mut collected: Vec<CollectedModel> = Vec::new();
    let mut seen_models: HashSet<String> = HashSet::new();
    let mut seen_tables: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let model = item.canonical_path().to_string();

        if !item.kind().is_struct() {
            return Err(ModelCodegenError::ModelNotAStruct { model });
        }

        if !seen_models.insert(model.clone()) {
            return Err(ModelCodegenError::DuplicateModelDeclaration { model });
        }

        let ModelArguments { table } = ModelArguments::parse(matched.args()?, &model)?;

        if !is_snake_case_identifier(&table) {
            return Err(ModelCodegenError::InvalidTableName { model, table });
        }

        let table_length = table.len();

        if table_length > POSTGRES_MAX_IDENTIFIER_BYTES {
            return Err(ModelCodegenError::TableNameTooLong {
                length: table_length,
                model,
                table,
            });
        }

        if let Some(first) = seen_tables.get(&table) {
            return Err(ModelCodegenError::DuplicateTableName {
                first: first.clone(),
                second: model,
                table,
            });
        }

        seen_tables.insert(table.clone(), model.clone());

        let mut scalar_columns: Vec<ResolvedColumn> = Vec::new();
        let mut deferred_foreign_keys: Vec<DeferredForeignKey> = Vec::new();
        let mut seen_columns: HashSet<String> = HashSet::new();

        for field in item.fields() {
            let column_attribute =
                select_unique_attribute(field.attributes(), &column_selector, || model.clone())?;
            let foreign_key_attribute =
                select_unique_attribute(field.attributes(), &foreign_key_selector, || {
                    model.clone()
                })?;

            match (column_attribute, foreign_key_attribute) {
                (None, None) => {
                    return Err(ModelCodegenError::UnattributedField {
                        field: field_display(field.identifier()),
                        model,
                    });
                }
                (None, Some(_)) => {
                    return Err(ModelCodegenError::ForeignKeyRequiresColumn {
                        field: field_display(field.identifier()),
                        model,
                    });
                }
                (Some(column_attribute), foreign_key_attribute) => {
                    let column_arguments = ColumnArguments::parse(column_attribute.args()?)?;

                    match foreign_key_attribute {
                        None => {
                            scalar_columns.push(resolve_scalar_column(
                                column_arguments,
                                field,
                                &model,
                                &mut seen_columns,
                            )?);
                        }
                        Some(_) => {
                            deferred_foreign_keys.push(defer_foreign_key(
                                column_arguments,
                                field,
                                &model,
                                index,
                                item,
                            )?);
                        }
                    }
                }
            }
        }

        let primary_key: Vec<ForeignKeyTargetColumn> = scalar_columns
            .iter()
            .filter(|column| column.primary_key)
            .map(|column| ForeignKeyTargetColumn {
                column_type: column.inferred.column_type.clone(),
                name: column.name.clone(),
            })
            .collect();

        targets.insert(
            item.canonical_path().clone(),
            ForeignKeyTarget {
                primary_key,
                table: table.clone(),
            },
        );

        collected.push(CollectedModel {
            deferred_foreign_keys,
            model,
            scalar_columns,
            seen_columns,
            table,
        });
    }

    Ok(collected)
}

fn resolve_model(
    collected: CollectedModel,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Model, ModelCodegenError> {
    let CollectedModel {
        deferred_foreign_keys,
        model,
        scalar_columns: mut columns,
        mut seen_columns,
        table,
    } = collected;

    let mut foreign_keys: Vec<ResolvedForeignKey> = Vec::new();

    for deferred in deferred_foreign_keys {
        let DeferredForeignKey {
            field_name,
            nullable,
            rust_type,
            target_path,
        } = deferred;

        let Some(target) = targets.get(&target_path) else {
            return Err(ModelCodegenError::ForeignKeyTargetNotAModel {
                field: field_name,
                model,
                rust_type,
            });
        };

        if target.primary_key.is_empty() {
            return Err(ModelCodegenError::ForeignKeyTargetWithoutPrimaryKey {
                field: field_name,
                model,
                target: target.table.clone(),
            });
        }

        let mut fk_columns: Vec<String> = Vec::new();
        let mut references_columns: Vec<String> = Vec::new();

        for referenced in &target.primary_key {
            let column_name = foreign_key_column_name(&field_name, &referenced.name);
            let column_name = register_column_name(column_name, &model, &mut seen_columns)?;

            columns.push(ResolvedColumn {
                inferred: InferredColumn {
                    column_type: referenced.column_type.clone(),
                    default: quote!(margaret_model::column_default::ColumnDefault::NotSet),
                    nullable,
                },
                name: column_name.clone(),
                primary_key: false,
            });

            fk_columns.push(column_name);
            references_columns.push(referenced.name.clone());
        }

        foreign_keys.push(ResolvedForeignKey {
            columns: fk_columns,
            references_columns,
            references_table: target.table.clone(),
        });
    }

    Ok(Model {
        columns,
        foreign_keys,
        table,
    })
}

pub(crate) fn models(index: &AttributeIndex) -> Result<Vec<Model>, ModelCodegenError> {
    let mut targets: HashMap<CanonicalPath, ForeignKeyTarget> = HashMap::new();
    let collected = collect_models(index, &mut targets)?;

    let mut resolved: Vec<Model> = collected
        .into_iter()
        .map(|collected_model| resolve_model(collected_model, &targets))
        .collect::<Result<Vec<Model>, ModelCodegenError>>()?;

    resolved.sort_by(|first, second| first.table.cmp(&second.table));

    Ok(resolved)
}
