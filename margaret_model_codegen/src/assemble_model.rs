use std::collections::HashSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::column_default::ColumnDefault;
use margaret_model::on_delete::OnDelete;
use margaret_schema_identifier_naming::index_name::index_name;
use margaret_schema_identifier_naming::primary_key_index_name::primary_key_index_name;
use margaret_schema_identifier_naming::unique_index_name::unique_index_name;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::assembled_model::AssembledModel;
use crate::collected_column::CollectedColumn;
use crate::collected_field::CollectedField;
use crate::collected_field_shape::CollectedFieldShape;
use crate::collected_model::CollectedModel;
use crate::declared_index::DeclaredIndex;
use crate::field_index::FieldIndex;
use crate::field_value::FieldValue;
use crate::index_kind::IndexKind;
use crate::key_column::KeyColumn;
use crate::key_reference::KeyReference;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_field::ModelField;
use crate::model_foreign_key::ModelForeignKey;
use crate::model_index::ModelIndex;
use crate::model_indexes::ModelIndexes;
use crate::primary_key_columns::PrimaryKeyColumns;
use crate::resolved_column::ResolvedColumn;

fn select_fields(fields: &[ModelField], names: &[String]) -> Vec<ModelField> {
    names
        .iter()
        .flat_map(|name| {
            fields
                .iter()
                .filter(move |field| &field.name == name)
                .cloned()
        })
        .collect()
}

fn column_field(
    collected: &CollectedField,
    start: usize,
    column: &CollectedColumn,
    primary_key: &[String],
    model: &str,
) -> Result<ModelField, ModelCodegenError> {
    let CollectedField { name, nullable, .. } = collected;
    let nullable = *nullable;
    let CollectedColumn {
        checks,
        column_type,
        default,
        name: column_name,
        value,
    } = column;

    if *default != ColumnDefault::NotSet && !primary_key.contains(name) {
        return Err(ModelCodegenError::ColumnDefaultOutsidePrimaryKey {
            field: name.clone(),
            model: model.to_string(),
        });
    }

    Ok(ModelField {
        columns: vec![ResolvedColumn {
            checks: checks.clone(),
            column_type: *column_type,
            default: *default,
            name: column_name.clone(),
            nullable,
        }],
        name: name.clone(),
        nullable,
        start,
        value: value.clone(),
    })
}

fn key_field(
    collected: &CollectedField,
    start: usize,
    on_delete: OnDelete,
    target: &CanonicalPath,
    reference: KeyReference,
) -> KeyField {
    let CollectedField { name, nullable, .. } = collected;
    let nullable = *nullable;
    let KeyReference {
        columns,
        references_columns,
        references_table,
    } = reference;

    KeyField {
        foreign_key: ModelForeignKey {
            columns: columns.iter().map(|column| column.name.clone()).collect(),
            field: name.clone(),
            on_delete,
            references_columns,
            references_table,
        },
        field: ModelField {
            columns: columns
                .into_iter()
                .map(|KeyColumn { column_type, name }| ResolvedColumn {
                    checks: Vec::new(),
                    column_type,
                    default: ColumnDefault::NotSet,
                    name,
                    nullable,
                })
                .collect(),
            name: name.clone(),
            nullable,
            start,
            value: FieldValue::Key {
                on_delete,
                target: target.clone(),
            },
        },
    }
}

fn model_fields(
    collected: &CollectedModel,
    primary_key: &[String],
    key_columns: &mut PrimaryKeyColumns,
    model: &str,
) -> Result<ModelFields, ModelCodegenError> {
    let mut fields: Vec<ModelField> = Vec::with_capacity(collected.fields.len());
    let mut foreign_keys: Vec<ModelForeignKey> = Vec::new();
    let mut start = 0;

    for collected_field in &collected.fields {
        let field = match &collected_field.shape {
            CollectedFieldShape::Column(column) => {
                column_field(collected_field, start, column, primary_key, model)?
            }
            CollectedFieldShape::Key { on_delete, target } => {
                let KeyField { field, foreign_key } = key_field(
                    collected_field,
                    start,
                    *on_delete,
                    target,
                    key_columns.reference(&collected_field.name, target, model)?,
                );

                foreign_keys.push(foreign_key);
                field
            }
        };

        start += field.columns.len();
        fields.push(field);
    }

    Ok(ModelFields {
        fields,
        foreign_keys,
    })
}

fn reject_duplicate_columns(fields: &[ModelField], model: &str) -> Result<(), ModelCodegenError> {
    let mut seen: HashSet<&str> = HashSet::new();

    for column in fields.iter().flat_map(|field| field.columns.iter()) {
        if !seen.insert(column.name.as_str()) {
            return Err(ModelCodegenError::DuplicateColumnName {
                column: column.name.clone(),
                model: model.to_string(),
            });
        }
    }

    Ok(())
}

fn unique_index(
    unique: Vec<ModelField>,
    table: &str,
    model: &str,
) -> Result<ModelIndex, ModelCodegenError> {
    if let Some(nullable) = unique.iter().find(|field| field.nullable) {
        return Err(ModelCodegenError::NullableUniqueField {
            field: nullable.name.clone(),
            model: model.to_string(),
        });
    }

    let columns: Vec<String> = unique
        .iter()
        .flat_map(|field| field.columns.iter().map(|column| column.name.clone()))
        .collect();

    Ok(ModelIndex {
        name: unique_index_name(table, &columns).map_err(|source| {
            ModelCodegenError::UniqueIndexNameTooLong {
                model: model.to_string(),
                source,
            }
        })?,
        fields: unique,
        kind: IndexKind::Unique,
    })
}

fn plain_indexes(
    collected: &CollectedModel,
    fields: &[ModelField],
    model: &str,
) -> Result<Vec<ModelIndex>, ModelCodegenError> {
    let mut indexes: Vec<ModelIndex> = Vec::new();

    for (collected_field, field) in collected.fields.iter().zip(fields) {
        let name = match &collected_field.index {
            FieldIndex::Absent => continue,
            FieldIndex::Derived => {
                let columns: Vec<String> = field
                    .columns
                    .iter()
                    .map(|column| column.name.clone())
                    .collect();

                index_name(&collected.table, &columns).map_err(|source| {
                    ModelCodegenError::IndexNameTooLong {
                        model: model.to_string(),
                        source,
                    }
                })?
            }
            FieldIndex::Explicit(name) => name.clone(),
        };

        indexes.push(ModelIndex {
            fields: vec![field.clone()],
            kind: IndexKind::Plain,
            name,
        });
    }

    for DeclaredIndex {
        fields: names,
        name,
    } in &collected.indexes
    {
        indexes.push(ModelIndex {
            fields: select_fields(fields, names),
            kind: IndexKind::Plain,
            name: name.clone(),
        });
    }

    indexes.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(indexes)
}

fn leads_a_key_of_kind(keys: &[&ModelIndex], kind: IndexKind, index: &ModelIndex) -> bool {
    keys.iter()
        .any(|key| key.kind == kind && key.fields.starts_with(&index.fields))
}

fn reject_conflicting_indexes(
    indexes: &ModelIndexes,
    model: &str,
) -> Result<(), ModelCodegenError> {
    let mut names: HashSet<&str> = HashSet::new();
    let keys: Vec<&ModelIndex> = indexes
        .all()
        .filter(|index| index.kind != IndexKind::Plain)
        .collect();

    for index in indexes.of_kind(IndexKind::Plain) {
        if !names.insert(index.name.as_str()) {
            return Err(ModelCodegenError::DuplicateIndexDeclaration {
                index: index.name.clone(),
                model: model.to_string(),
            });
        }

        if leads_a_key_of_kind(&keys, IndexKind::PrimaryKey, index) {
            return Err(ModelCodegenError::RedundantIndexOnPrimaryKeyColumns {
                columns: index.column_names().join(", "),
                model: model.to_string(),
            });
        }

        if leads_a_key_of_kind(&keys, IndexKind::Unique, index) {
            return Err(ModelCodegenError::RedundantIndexOnUniqueColumns {
                columns: index.column_names().join(", "),
                model: model.to_string(),
            });
        }
    }

    for index in indexes.all() {
        if let Some(extended) = keys.iter().find(|key| {
            index.fields.len() > key.fields.len() && index.fields.starts_with(&key.fields)
        }) {
            return Err(ModelCodegenError::IndexExtendsUniqueKey {
                index: index.field_names().join(", "),
                model: model.to_string(),
                unique: extended.field_names().join(", "),
            });
        }
    }

    Ok(())
}

fn reject_unindexed_foreign_keys(
    foreign_keys: &[ModelForeignKey],
    indexes: &ModelIndexes,
    model: &str,
) -> Result<(), ModelCodegenError> {
    match foreign_keys.iter().find(|foreign_key| {
        !indexes.all().any(|index| {
            index
                .fields
                .first()
                .is_some_and(|leading| leading.name == foreign_key.field)
        })
    }) {
        Some(unindexed) => Err(ModelCodegenError::UnindexedForeignKey {
            field: unindexed.field.clone(),
            model: model.to_string(),
        }),
        None => Ok(()),
    }
}

struct KeyField {
    field: ModelField,
    foreign_key: ModelForeignKey,
}

struct ModelFields {
    fields: Vec<ModelField>,
    foreign_keys: Vec<ModelForeignKey>,
}

pub(crate) fn assemble_model(
    collected: &CollectedModel,
    key_columns: &mut PrimaryKeyColumns,
    namespace: TableNamespace,
) -> Result<AssembledModel, ModelCodegenError> {
    let model = collected.path.to_string();
    let primary_key = collected.primary_key_fields();
    let ModelFields {
        fields,
        foreign_keys,
    } = model_fields(collected, &primary_key, key_columns, &model)?;

    reject_duplicate_columns(&fields, &model)?;

    let mut secondary: Vec<ModelIndex> = Vec::new();

    for (collected_field, field) in collected.fields.iter().zip(&fields) {
        if collected_field.unique {
            secondary.push(unique_index(vec![field.clone()], &collected.table, &model)?);
        }
    }

    for unique in &collected.uniques {
        secondary.push(unique_index(
            select_fields(&fields, unique),
            &collected.table,
            &model,
        )?);
    }

    secondary.extend(plain_indexes(collected, &fields, &model)?);

    let indexes = ModelIndexes {
        primary_key: ModelIndex {
            fields: select_fields(&fields, &primary_key),
            kind: IndexKind::PrimaryKey,
            name: primary_key_index_name(&collected.table).map_err(|source| {
                ModelCodegenError::PrimaryKeyIndexNameTooLong {
                    model: model.clone(),
                    source,
                }
            })?,
        },
        secondary,
    };

    reject_conflicting_indexes(&indexes, &model)?;
    reject_unindexed_foreign_keys(&foreign_keys, &indexes, &model)?;

    Ok(AssembledModel {
        foreign_keys,
        fields,
        indexes,
        module: collected.module.clone(),
        namespace,
        path: collected.path.clone(),
        relations: collected.relations.clone(),
        table: collected.table.clone(),
    })
}
