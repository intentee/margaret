use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use quote::quote;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_unique_attribute::select_unique_attribute;
use margaret_schema_identifier_naming::index_name::index_name;
use margaret_schema_identifier_naming::schema_identifier::schema_identifier;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;
use margaret_toposort::topological_order::topological_order;

use crate::collected_model::CollectedModel;
use crate::column_arguments::ColumnArguments;
use crate::deferred_foreign_key::DeferredForeignKey;
use crate::foreign_key_arguments::ForeignKeyArguments;
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
use crate::resolved_index::ResolvedIndex;

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

    if !seen_columns.insert(column_name.clone()) {
        return Err(ModelCodegenError::DuplicateColumnName {
            column: column_name,
            model: model.to_string(),
        });
    }

    Ok(column_name)
}

fn resolve_index(
    table: &str,
    column: &str,
    model: &str,
) -> Result<ResolvedIndex, ModelCodegenError> {
    let name = index_name(table, column).map_err(|source| ModelCodegenError::IndexNameTooLong {
        model: model.to_string(),
        source,
    })?;

    Ok(ResolvedIndex {
        column: column.to_string(),
        name,
    })
}

fn resolve_scalar_column(
    ColumnArguments {
        name,
        primary_key,
        unique,
    }: ColumnArguments,
    index: bool,
    field: &IndexedField,
    model: &str,
    seen_columns: &mut HashSet<String>,
) -> Result<ResolvedColumn, ModelCodegenError> {
    let column_name = resolve_column_name(name, field.identifier(), model)?;

    if let Err(source) = validate_identifier_length(&column_name) {
        return Err(ModelCodegenError::ColumnNameTooLong {
            model: model.to_string(),
            source,
        });
    }

    let column_name = register_column_name(column_name, model, seen_columns)?;

    if index && unique {
        return Err(ModelCodegenError::RedundantIndexOnUniqueColumn {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    if index && primary_key {
        return Err(ModelCodegenError::RedundantIndexOnPrimaryKeyColumn {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    let inferred = infer_column_type(field.ty(), model, &column_name)?;

    Ok(ResolvedColumn {
        index,
        inferred,
        name: column_name,
        primary_key,
        unique,
    })
}

fn defer_foreign_key(
    ColumnArguments {
        name,
        primary_key,
        unique,
    }: ColumnArguments,
    index: bool,
    foreign_key_arguments: &AttributeArgs,
    field: &IndexedField,
    model: &str,
    attribute_index: &AttributeIndex,
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

    if index && unique {
        return Err(ModelCodegenError::RedundantIndexOnUniqueColumn {
            field: field_name,
            model: model.to_string(),
        });
    }

    let ForeignKeyArguments { on_delete } =
        ForeignKeyArguments::parse(foreign_key_arguments, model, &field_name)?;

    let (target_type, nullable) = match option_inner(field.ty()) {
        Some(inner) => (inner, true),
        None => (field.ty(), false),
    };

    let rust_type = field.ty().to_token_stream().to_string();

    let target_path = attribute_index
        .resolve_item_type(item, target_type)
        .ok_or_else(|| ModelCodegenError::ForeignKeyTargetNotAModel {
            field: field_name.clone(),
            model: model.to_string(),
            rust_type: rust_type.clone(),
        })?;

    Ok(DeferredForeignKey {
        field_name,
        index,
        nullable,
        on_delete,
        rust_type,
        target_path,
        unique,
    })
}

fn collect_models(
    attribute_index: &AttributeIndex,
    targets: &mut HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Vec<CollectedModel>, ModelCodegenError> {
    let selector = AttributeSelector::from_marker("model");
    let column_selector = AttributeSelector::from_marker("column");
    let foreign_key_selector = AttributeSelector::from_marker("foreign_key");
    let index_selector = AttributeSelector::from_marker("index");
    let mut collected: Vec<CollectedModel> = Vec::new();
    let mut seen_models: HashSet<String> = HashSet::new();
    let mut seen_tables: HashMap<String, String> = HashMap::new();

    for matched in attribute_index.select(&selector) {
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

        if let Err(source) = validate_identifier_length(&table) {
            return Err(ModelCodegenError::TableNameTooLong { model, source });
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
            let index =
                select_unique_attribute(field.attributes(), &index_selector, || model.clone())?
                    .is_some();

            if index && column_attribute.is_none() {
                return Err(ModelCodegenError::IndexRequiresColumn {
                    field: field_display(field.identifier()),
                    model,
                });
            }

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
                                index,
                                field,
                                &model,
                                &mut seen_columns,
                            )?);
                        }
                        Some(foreign_key_attribute) => {
                            deferred_foreign_keys.push(defer_foreign_key(
                                column_arguments,
                                index,
                                foreign_key_attribute.args()?,
                                field,
                                &model,
                                attribute_index,
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
    let mut indexes: Vec<ResolvedIndex> = Vec::new();

    for deferred in deferred_foreign_keys {
        let DeferredForeignKey {
            field_name,
            index,
            nullable,
            on_delete,
            rust_type,
            target_path,
            unique,
        } = deferred;

        let Some(target) = targets.get(&target_path) else {
            return Err(ModelCodegenError::ForeignKeyTargetNotAModel {
                field: field_name,
                model,
                rust_type,
            });
        };

        let referenced = match target.primary_key.as_slice() {
            [] => {
                return Err(ModelCodegenError::ForeignKeyTargetWithoutPrimaryKey {
                    field: field_name,
                    model,
                    target: target.table.clone(),
                });
            }
            [referenced] => referenced,
            _ => {
                return Err(ModelCodegenError::ForeignKeyTargetHasCompositePrimaryKey {
                    field: field_name,
                    model,
                    target: target.table.clone(),
                });
            }
        };

        let column_name = match schema_identifier(&[field_name.as_str(), referenced.name.as_str()])
        {
            Ok(column_name) => column_name,
            Err(source) => {
                return Err(ModelCodegenError::ForeignKeyColumnNameTooLong {
                    field: field_name,
                    model,
                    source,
                });
            }
        };
        let column_name = register_column_name(column_name, &model, &mut seen_columns)?;

        columns.push(ResolvedColumn {
            index,
            inferred: InferredColumn {
                column_type: referenced.column_type.clone(),
                default: quote!(margaret_model::column_default::ColumnDefault::NotSet),
                nullable,
            },
            name: column_name.clone(),
            primary_key: false,
            unique: false,
        });

        foreign_keys.push(ResolvedForeignKey {
            column: column_name,
            on_delete,
            references_column: referenced.name.clone(),
            references_table: target.table.clone(),
            unique,
        });
    }

    for column in &columns {
        if column.index {
            indexes.push(resolve_index(&table, &column.name, &model)?);
        }
    }

    Ok(Model {
        columns,
        foreign_keys,
        indexes,
        table,
    })
}

fn order_by_dependencies(resolved: Vec<Model>) -> Result<Vec<Model>, ModelCodegenError> {
    let dependencies: BTreeMap<String, BTreeSet<String>> = resolved
        .iter()
        .map(|model| {
            let references: BTreeSet<String> = model
                .foreign_keys
                .iter()
                .map(|foreign_key| foreign_key.references_table.clone())
                .filter(|referenced_table| referenced_table != &model.table)
                .collect();

            (model.table.clone(), references)
        })
        .collect();

    let order =
        topological_order(&dependencies).map_err(|cycle| ModelCodegenError::ForeignKeyCycle {
            path: cycle.path.join(" -> "),
        })?;

    let mut by_table: HashMap<String, Model> = resolved
        .into_iter()
        .map(|model| (model.table.clone(), model))
        .collect();

    Ok(order
        .iter()
        .filter_map(|table| by_table.remove(table))
        .collect())
}

pub fn models(attribute_index: &AttributeIndex) -> Result<Vec<Model>, ModelCodegenError> {
    let mut targets: HashMap<CanonicalPath, ForeignKeyTarget> = HashMap::new();
    let collected = collect_models(attribute_index, &mut targets)?;

    let resolved: Vec<Model> = collected
        .into_iter()
        .map(|collected_model| resolve_model(collected_model, &targets))
        .collect::<Result<Vec<Model>, ModelCodegenError>>()?;

    order_by_dependencies(resolved)
}
