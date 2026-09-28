use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_framework_attributes::select_framework_attributes;
use margaret_attributes::select_unique_framework_attribute::select_unique_framework_attribute;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_model::column_default::ColumnDefault;
use margaret_schema_identifier_naming::primary_key_index_name::primary_key_index_name;
use margaret_schema_identifier_naming::schema_identifier::schema_identifier;
use margaret_schema_identifier_naming::unique_index_name::unique_index_name;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;
use margaret_syn_type_peeling::peel_standard_wrapper::peel_standard_wrapper;
use margaret_toposort::topological_order::topological_order;

use crate::collected_model::CollectedModel;
use crate::column_arguments::ColumnArguments;
use crate::column_type_context::ColumnTypeContext;
use crate::declared_column_type::DeclaredColumnType;
use crate::deferred_foreign_key::DeferredForeignKey;
use crate::deferred_model_foreign_key::DeferredModelForeignKey;
use crate::explicit_index_name::explicit_index_name;
use crate::field_index::FieldIndex;
use crate::foreign_key_arguments::ForeignKeyArguments;
use crate::foreign_key_target::ForeignKeyTarget;
use crate::foreign_key_target_column::ForeignKeyTargetColumn;
use crate::index_arguments::IndexArguments;
use crate::index_redundancy::IndexRedundancy;
use crate::infer_column::infer_column;
use crate::inferred_column::InferredColumn;
use crate::model::Model;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_foreign_key_arguments::ModelForeignKeyArguments;
use crate::model_index_arguments::ModelIndexArguments;
use crate::model_primary_key_arguments::ModelPrimaryKeyArguments;
use crate::model_unique_arguments::ModelUniqueArguments;
use crate::numeric_digits::NumericDigits;
use crate::redundant_index::redundant_index;
use crate::resolve_declared_columns::resolve_declared_columns;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;
use crate::scalar_column::ScalarColumn;

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

fn resolve_field_index(field: &IndexedField, model: &str) -> Result<FieldIndex, ModelCodegenError> {
    match select_framework_attributes(field.attributes(), FrameworkAttribute::Index).as_slice() {
        [] => Ok(FieldIndex::Absent),
        [declared] => {
            let IndexArguments { name } =
                IndexArguments::parse(declared.args()?, model, &field_display(field.identifier()))?;

            match name {
                None => Ok(FieldIndex::Derived),
                Some(name) => Ok(FieldIndex::Explicit(explicit_index_name(name, model)?)),
            }
        }
        _ => Err(ModelCodegenError::RepeatedFieldIndex {
            field: field_display(field.identifier()),
            model: model.to_string(),
        }),
    }
}

fn ensure_index_not_redundant(
    index: &ResolvedIndex,
    primary_key: &[String],
    unique_constraints: &[ResolvedUniqueConstraint],
    model: &str,
) -> Result<(), ModelCodegenError> {
    match redundant_index(index, primary_key, unique_constraints) {
        Some(IndexRedundancy::PrimaryKeyLeadingColumns) => {
            Err(ModelCodegenError::RedundantIndexOnPrimaryKeyColumns {
                columns: index.columns.join(", "),
                model: model.to_string(),
            })
        }
        Some(IndexRedundancy::UniqueConstraint) => {
            Err(ModelCodegenError::RedundantIndexOnUniqueColumns {
                columns: index.columns.join(", "),
                model: model.to_string(),
            })
        }
        None => Ok(()),
    }
}

fn register_table_name(
    seen_table_names: &mut HashMap<String, String>,
    table: String,
    model: &str,
) -> Result<(), ModelCodegenError> {
    if let Some(first) = seen_table_names.get(&table) {
        return Err(ModelCodegenError::DuplicateTableName {
            first: first.clone(),
            second: model.to_string(),
            table,
        });
    }

    seen_table_names.insert(table, model.to_string());

    Ok(())
}

fn register_constraint_index_name(
    seen_table_names: &HashMap<String, String>,
    constraint_index_names: &mut HashMap<String, String>,
    name: String,
    table: &str,
) -> Result<(), ModelCodegenError> {
    if let Some(table_model) = seen_table_names.get(&name) {
        return Err(ModelCodegenError::ConstraintIndexCollidesWithTableName {
            constraint_table: table.to_string(),
            name,
            table_model: table_model.clone(),
        });
    }

    if let Some(first_table) = constraint_index_names.get(&name) {
        return Err(ModelCodegenError::DuplicateConstraintIndexName {
            first_table: first_table.clone(),
            name,
            second_table: table.to_string(),
        });
    }

    constraint_index_names.insert(name, table.to_string());

    Ok(())
}

fn register_index_name(
    seen_table_names: &HashMap<String, String>,
    constraint_index_names: &HashMap<String, String>,
    index_names: &mut HashMap<String, String>,
    name: String,
    table: &str,
) -> Result<(), ModelCodegenError> {
    if let Some(table_model) = seen_table_names.get(&name) {
        return Err(ModelCodegenError::IndexNameCollidesWithTableName {
            index_table: table.to_string(),
            name,
            table_model: table_model.clone(),
        });
    }

    if let Some(constraint_table) = constraint_index_names.get(&name) {
        return Err(ModelCodegenError::IndexNameCollidesWithConstraintIndex {
            constraint_table: constraint_table.clone(),
            index_table: table.to_string(),
            name,
        });
    }

    if let Some(first) = index_names.get(&name) {
        return Err(ModelCodegenError::DuplicateIndexName {
            first: first.clone(),
            name,
            second: table.to_string(),
        });
    }

    index_names.insert(name, table.to_string());

    Ok(())
}

fn resolve_scalar_column(
    ColumnArguments {
        check,
        name,
        numeric_digits,
        primary_key,
        unique,
    }: ColumnArguments,
    field: &IndexedField,
    model: &str,
    table: &str,
    seen_columns: &mut HashSet<String>,
    mut column_type_context: ColumnTypeContext,
) -> Result<ScalarColumn, ModelCodegenError> {
    let column_name = resolve_column_name(name, field.identifier(), model)?;

    if let Err(source) = validate_identifier_length(&column_name) {
        return Err(ModelCodegenError::ColumnNameTooLong {
            model: model.to_string(),
            source,
        });
    }

    let column_name = register_column_name(column_name, model, seen_columns)?;

    let declared_column_type = DeclaredColumnType::of(
        column_type_context.attribute_index,
        column_type_context.item,
        field.ty(),
    );
    let inferred = infer_column(
        &mut column_type_context,
        &declared_column_type,
        &numeric_digits,
        model,
        &column_name,
    )?;

    let checks = check.resolve(inferred.column_type, table, &column_name, model)?;

    Ok(ScalarColumn {
        column: ResolvedColumn {
            checks,
            inferred,
            name: column_name,
            primary_key,
        },
        unique,
    })
}

fn defer_foreign_key(
    ColumnArguments {
        check,
        name,
        numeric_digits,
        primary_key,
        unique,
    }: ColumnArguments,
    index: FieldIndex,
    field: &IndexedField,
    foreign_key_arguments: &AttributeArgs,
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

    if check.is_declared() {
        return Err(ModelCodegenError::ForeignKeyCannotDeclareCheckConstraint {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    if matches!(numeric_digits, NumericDigits::Declared { .. }) {
        return Err(ModelCodegenError::ForeignKeyCannotDeclareNumericDigits {
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
        FieldIdentifier::Positional(field_position) => {
            return Err(ModelCodegenError::ForeignKeyRequiresNamedField {
                model: model.to_string(),
                position: *field_position,
            });
        }
    };

    let ForeignKeyArguments { on_delete } =
        ForeignKeyArguments::parse(foreign_key_arguments, model, &field_name)?;

    let DeclaredColumnType {
        base: after_option,
        declared: _,
        nullable,
    } = DeclaredColumnType::of(attribute_index, item, field.ty());

    let (target_type, indirected) = match peel_standard_wrapper(
        attribute_index,
        item,
        after_option,
        &[
            StandardLibraryItem::Arc,
            StandardLibraryItem::Box,
            StandardLibraryItem::Rc,
        ],
    ) {
        Some(inner) => (inner, true),
        None => (after_option, false),
    };

    let rust_type = field.ty().to_token_stream().to_string();

    let target_path = attribute_index
        .resolve_item_type(item, target_type)
        .ok_or_else(|| ModelCodegenError::ForeignKeyTargetNotAModel {
            field: field_name.clone(),
            model: model.to_string(),
            rust_type: rust_type.clone(),
        })?;

    if &target_path == item.canonical_path() && !indirected {
        return Err(
            ModelCodegenError::SelfReferentialForeignKeyRequiresIndirection {
                field: field_name,
                model: model.to_string(),
            },
        );
    }

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

struct ModelColumns {
    deferred_foreign_keys: Vec<DeferredForeignKey>,
    indexes: Vec<ResolvedIndex>,
    scalar_columns: Vec<ResolvedColumn>,
    seen_columns: HashSet<String>,
    unique_constraints: Vec<ResolvedUniqueConstraint>,
}

fn reject_model_attribute_on_a_field(
    field: &IndexedField,
    model: &str,
) -> Result<(), ModelCodegenError> {
    if !select_framework_attributes(field.attributes(), FrameworkAttribute::PrimaryKey).is_empty() {
        return Err(ModelCodegenError::PrimaryKeyIsNotAFieldAttribute {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    if !select_framework_attributes(field.attributes(), FrameworkAttribute::Unique).is_empty() {
        return Err(ModelCodegenError::UniqueIsNotAFieldAttribute {
            field: field_display(field.identifier()),
            model: model.to_string(),
        });
    }

    Ok(())
}

fn collect_model_columns(
    attribute_index: &AttributeIndex,
    item: &IndexedItem,
    model: &str,
    table: &str,
    validated_enums: &mut HashSet<CanonicalPath>,
) -> Result<ModelColumns, ModelCodegenError> {
    let mut collected = ModelColumns {
        deferred_foreign_keys: Vec::new(),
        indexes: Vec::new(),
        scalar_columns: Vec::new(),
        seen_columns: HashSet::new(),
        unique_constraints: Vec::new(),
    };

    for field in item.fields() {
        reject_model_attribute_on_a_field(field, model)?;

        let column_attribute = select_unique_framework_attribute(
            field.attributes(),
            FrameworkAttribute::Column,
            || model.to_string(),
        )?;
        let foreign_key_attribute = select_unique_framework_attribute(
            field.attributes(),
            FrameworkAttribute::ForeignKey,
            || model.to_string(),
        )?;
        let index = resolve_field_index(field, model)?;

        if matches!(index, FieldIndex::Absent) {
        } else if column_attribute.is_none() {
            return Err(ModelCodegenError::IndexRequiresColumn {
                field: field_display(field.identifier()),
                model: model.to_string(),
            });
        }

        match (column_attribute, foreign_key_attribute) {
            (None, None) => {
                return Err(ModelCodegenError::UnattributedField {
                    field: field_display(field.identifier()),
                    model: model.to_string(),
                });
            }
            (None, Some(_)) => {
                return Err(ModelCodegenError::ForeignKeyRequiresColumn {
                    field: field_display(field.identifier()),
                    model: model.to_string(),
                });
            }
            (Some(column_attribute), foreign_key_attribute) => {
                let column_arguments = ColumnArguments::parse(
                    column_attribute.args()?,
                    model,
                    &field_display(field.identifier()),
                )?;

                match foreign_key_attribute {
                    None => {
                        let ScalarColumn { column, unique } = resolve_scalar_column(
                            column_arguments,
                            field,
                            model,
                            table,
                            &mut collected.seen_columns,
                            ColumnTypeContext {
                                attribute_index,
                                item,
                                validated_enums,
                            },
                        )?;

                        if unique {
                            collected.unique_constraints.push(ResolvedUniqueConstraint {
                                columns: vec![column.name.clone()],
                            });
                        }

                        index.extend(
                            &mut collected.indexes,
                            vec![column.name.clone()],
                            table,
                            model,
                        )?;
                        collected.scalar_columns.push(column);
                    }
                    Some(foreign_key_attribute) => {
                        collected.deferred_foreign_keys.push(defer_foreign_key(
                            column_arguments,
                            index,
                            field,
                            foreign_key_attribute.args()?,
                            model,
                            attribute_index,
                            item,
                        )?);
                    }
                }
            }
        }
    }

    Ok(collected)
}

fn resolve_primary_key<'columns>(
    item: &IndexedItem,
    scalar_columns: &'columns [ResolvedColumn],
    model: &str,
) -> Result<Vec<&'columns ResolvedColumn>, ModelCodegenError> {
    let declarations =
        select_framework_attributes(item.attributes(), FrameworkAttribute::PrimaryKey);
    let flagged: Vec<&ResolvedColumn> = scalar_columns
        .iter()
        .filter(|column| column.primary_key)
        .collect();

    match (declarations.as_slice(), flagged.as_slice()) {
        ([], []) => Ok(Vec::new()),
        ([], [column]) => Ok(vec![*column]),
        ([], _) => Err(
            ModelCodegenError::CompositePrimaryKeyRequiresModelDeclaration {
                model: model.to_string(),
            },
        ),
        ([_], [_, ..]) => Err(ModelCodegenError::ConflictingPrimaryKeyDeclarations {
            model: model.to_string(),
        }),
        ([declaration], []) => {
            let ModelPrimaryKeyArguments { columns } =
                ModelPrimaryKeyArguments::parse(declaration.args()?, model)?;

            resolve_declared_columns(
                &columns,
                scalar_columns,
                FrameworkAttribute::PrimaryKey,
                model,
            )
        }
        (_, _) => Err(ModelCodegenError::DuplicateModelPrimaryKey {
            model: model.to_string(),
        }),
    }
}

fn collect_model_unique_constraints(
    item: &IndexedItem,
    model: &str,
) -> Result<Vec<ModelUniqueArguments>, ModelCodegenError> {
    let mut collected: Vec<ModelUniqueArguments> = Vec::new();
    let mut seen: HashSet<Vec<String>> = HashSet::new();

    for attribute in select_framework_attributes(item.attributes(), FrameworkAttribute::Unique) {
        let declared = ModelUniqueArguments::parse(attribute.args()?, model)?;

        if !seen.insert(declared.columns.clone()) {
            return Err(ModelCodegenError::DuplicateModelUniqueConstraint {
                columns: declared.columns.join(", "),
                model: model.to_string(),
            });
        }

        collected.push(declared);
    }

    Ok(collected)
}

fn collect_model_indexes(
    item: &IndexedItem,
    model: &str,
) -> Result<Vec<ModelIndexArguments>, ModelCodegenError> {
    let mut collected: Vec<ModelIndexArguments> = Vec::new();

    for attribute in select_framework_attributes(item.attributes(), FrameworkAttribute::Index) {
        collected.push(ModelIndexArguments::parse(attribute.args()?, model)?);
    }

    Ok(collected)
}

fn collect_model_foreign_keys(
    attribute_index: &AttributeIndex,
    item: &IndexedItem,
    model: &str,
) -> Result<Vec<DeferredModelForeignKey>, ModelCodegenError> {
    let mut deferred: Vec<DeferredModelForeignKey> = Vec::new();

    for attribute in select_framework_attributes(item.attributes(), FrameworkAttribute::ForeignKey)
    {
        let ModelForeignKeyArguments {
            columns,
            on_delete,
            references: declared_references,
        } = ModelForeignKeyArguments::parse(attribute.args()?, model)?;

        let references = format_path(&declared_references);
        let target_path = attribute_index
            .resolve_item_path(item, &declared_references)
            .ok_or_else(|| ModelCodegenError::ModelForeignKeyTargetNotAModel {
                model: model.to_string(),
                references: references.clone(),
            })?;

        deferred.push(DeferredModelForeignKey {
            columns,
            on_delete,
            references,
            target_path,
        });
    }

    Ok(deferred)
}

fn collect_models(
    attribute_index: &AttributeIndex,
    targets: &mut HashMap<CanonicalPath, ForeignKeyTarget>,
    seen_table_names: &mut HashMap<String, String>,
) -> Result<Vec<CollectedModel>, ModelCodegenError> {
    let mut collected: Vec<CollectedModel> = Vec::new();
    let mut seen_models: HashSet<String> = HashSet::new();
    let mut validated_enums: HashSet<CanonicalPath> = HashSet::new();

    for matched in attribute_index.select_framework_attribute(FrameworkAttribute::Model) {
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

        register_table_name(seen_table_names, table.clone(), &model)?;

        let ModelColumns {
            deferred_foreign_keys,
            indexes,
            scalar_columns,
            seen_columns,
            unique_constraints,
        } = collect_model_columns(attribute_index, item, &model, &table, &mut validated_enums)?;

        let deferred_model_foreign_keys =
            collect_model_foreign_keys(attribute_index, item, &model)?;
        let deferred_model_indexes = collect_model_indexes(item, &model)?;
        let deferred_model_unique_constraints = collect_model_unique_constraints(item, &model)?;
        let primary_key_columns = resolve_primary_key(item, &scalar_columns, &model)?;

        let target_primary_key: Vec<ForeignKeyTargetColumn> = primary_key_columns
            .iter()
            .map(|column| ForeignKeyTargetColumn {
                column_type: column.inferred.column_type,
                name: column.name.clone(),
            })
            .collect();

        let primary_key: Vec<String> = primary_key_columns
            .iter()
            .map(|column| column.name.clone())
            .collect();

        targets.insert(
            item.canonical_path().clone(),
            ForeignKeyTarget {
                primary_key: target_primary_key,
                table: table.clone(),
            },
        );

        collected.push(CollectedModel {
            deferred_foreign_keys,
            deferred_model_foreign_keys,
            deferred_model_indexes,
            deferred_model_unique_constraints,
            indexes,
            model,
            primary_key,
            scalar_columns,
            seen_columns,
            table,
            unique_constraints,
        });
    }

    Ok(collected)
}

fn resolve_indexes(
    indexes: Vec<ResolvedIndex>,
    primary_key: &[String],
    unique_constraints: &[ResolvedUniqueConstraint],
    model: &str,
) -> Result<Vec<ResolvedIndex>, ModelCodegenError> {
    let mut seen: HashSet<String> = HashSet::new();

    for index in &indexes {
        if !seen.insert(index.name.clone()) {
            return Err(ModelCodegenError::DuplicateIndexDeclaration {
                index: index.name.clone(),
                model: model.to_string(),
            });
        }

        ensure_index_not_redundant(index, primary_key, unique_constraints, model)?;
    }

    let mut resolved = indexes;

    resolved.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(resolved)
}

struct ResolvedForeignKeys {
    columns: Vec<ResolvedColumn>,
    foreign_keys: Vec<ResolvedForeignKey>,
    indexes: Vec<ResolvedIndex>,
    unique_constraints: Vec<ResolvedUniqueConstraint>,
}

fn resolve_foreign_keys(
    deferred_foreign_keys: Vec<DeferredForeignKey>,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
    model: &str,
    table: &str,
    seen_columns: &mut HashSet<String>,
) -> Result<ResolvedForeignKeys, ModelCodegenError> {
    let mut resolved = ResolvedForeignKeys {
        columns: Vec::new(),
        foreign_keys: Vec::new(),
        indexes: Vec::new(),
        unique_constraints: Vec::new(),
    };

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
                model: model.to_string(),
                rust_type,
            });
        };

        if target.primary_key.is_empty() {
            return Err(ModelCodegenError::ForeignKeyTargetWithoutPrimaryKey {
                field: field_name,
                model: model.to_string(),
                target: target.table.clone(),
            });
        }

        let mut derived_columns: Vec<String> = Vec::with_capacity(target.primary_key.len());

        for referenced in &target.primary_key {
            let column_name =
                match schema_identifier(&[field_name.as_str(), referenced.name.as_str()]) {
                    Ok(column_name) => column_name,
                    Err(source) => {
                        return Err(ModelCodegenError::ForeignKeyColumnNameTooLong {
                            field: field_name,
                            model: model.to_string(),
                            source,
                        });
                    }
                };
            let column_name = register_column_name(column_name, model, seen_columns)?;

            resolved.columns.push(ResolvedColumn {
                checks: Vec::new(),
                inferred: InferredColumn {
                    column_type: referenced.column_type,
                    default: ColumnDefault::NotSet,
                    nullable,
                },
                name: column_name.clone(),
                primary_key: false,
            });
            derived_columns.push(column_name);
        }

        if unique {
            resolved.unique_constraints.push(ResolvedUniqueConstraint {
                columns: derived_columns.clone(),
            });
        }

        index.extend(&mut resolved.indexes, derived_columns.clone(), table, model)?;

        resolved.foreign_keys.push(ResolvedForeignKey {
            columns: derived_columns,
            on_delete,
            references_columns: target
                .primary_key
                .iter()
                .map(|referenced| referenced.name.clone())
                .collect(),
            references_table: target.table.clone(),
        });
    }

    Ok(resolved)
}

fn resolve_model_foreign_keys(
    deferred_model_foreign_keys: Vec<DeferredModelForeignKey>,
    columns: &[ResolvedColumn],
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
    model: &str,
) -> Result<Vec<ResolvedForeignKey>, ModelCodegenError> {
    let mut resolved: Vec<ResolvedForeignKey> = Vec::new();
    let mut seen_column_lists: HashSet<Vec<String>> = HashSet::new();

    for deferred in deferred_model_foreign_keys {
        let DeferredModelForeignKey {
            columns: declared_columns,
            on_delete,
            references,
            target_path,
        } = deferred;

        let Some(target) = targets.get(&target_path) else {
            return Err(ModelCodegenError::ModelForeignKeyTargetNotAModel {
                model: model.to_string(),
                references,
            });
        };

        if target.primary_key.is_empty() {
            return Err(ModelCodegenError::ModelForeignKeyTargetWithoutPrimaryKey {
                model: model.to_string(),
                target: target.table.clone(),
            });
        }

        if declared_columns.len() != target.primary_key.len() {
            return Err(ModelCodegenError::ModelForeignKeyArityMismatch {
                declared: declared_columns.len(),
                expected: target.primary_key.len(),
                model: model.to_string(),
                target: target.table.clone(),
            });
        }

        if !seen_column_lists.insert(declared_columns.clone()) {
            return Err(ModelCodegenError::DuplicateModelForeignKey {
                columns: declared_columns.join(", "),
                model: model.to_string(),
            });
        }

        let constrained = resolve_declared_columns(
            &declared_columns,
            columns,
            FrameworkAttribute::ForeignKey,
            model,
        )?;

        for (column, referenced) in constrained.iter().zip(&target.primary_key) {
            if column.inferred.column_type != referenced.column_type {
                return Err(ModelCodegenError::ModelForeignKeyColumnTypeMismatch {
                    column: column.name.clone(),
                    model: model.to_string(),
                    target: target.table.clone(),
                    target_column: referenced.name.clone(),
                });
            }
        }

        resolved.push(ResolvedForeignKey {
            columns: declared_columns,
            on_delete,
            references_columns: target
                .primary_key
                .iter()
                .map(|referenced| referenced.name.clone())
                .collect(),
            references_table: target.table.clone(),
        });
    }

    Ok(resolved)
}

fn resolve_model_unique_constraints(
    declared: Vec<ModelUniqueArguments>,
    columns: &[ResolvedColumn],
    model: &str,
) -> Result<Vec<ResolvedUniqueConstraint>, ModelCodegenError> {
    let mut resolved: Vec<ResolvedUniqueConstraint> = Vec::with_capacity(declared.len());

    for ModelUniqueArguments {
        columns: declared_columns,
    } in declared
    {
        resolve_declared_columns(
            &declared_columns,
            columns,
            FrameworkAttribute::Unique,
            model,
        )?;

        resolved.push(ResolvedUniqueConstraint {
            columns: declared_columns,
        });
    }

    Ok(resolved)
}

fn resolve_model_indexes(
    declared: Vec<ModelIndexArguments>,
    columns: &[ResolvedColumn],
    model: &str,
) -> Result<Vec<ResolvedIndex>, ModelCodegenError> {
    let mut resolved: Vec<ResolvedIndex> = Vec::with_capacity(declared.len());

    for ModelIndexArguments {
        columns: declared_columns,
        name,
    } in declared
    {
        resolve_declared_columns(&declared_columns, columns, FrameworkAttribute::Index, model)?;

        resolved.push(ResolvedIndex {
            columns: declared_columns,
            name,
        });
    }

    Ok(resolved)
}

fn resolve_model(
    collected: CollectedModel,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Model, ModelCodegenError> {
    let CollectedModel {
        deferred_foreign_keys,
        deferred_model_foreign_keys,
        deferred_model_indexes,
        deferred_model_unique_constraints,
        indexes: mut collected_indexes,
        model,
        primary_key,
        scalar_columns: mut columns,
        mut seen_columns,
        table,
        unique_constraints: mut collected_unique_constraints,
    } = collected;

    let ResolvedForeignKeys {
        columns: foreign_key_columns,
        mut foreign_keys,
        indexes: foreign_key_indexes,
        unique_constraints: foreign_key_unique_constraints,
    } = resolve_foreign_keys(
        deferred_foreign_keys,
        targets,
        &model,
        &table,
        &mut seen_columns,
    )?;

    columns.extend(foreign_key_columns);
    collected_indexes.extend(foreign_key_indexes);
    collected_unique_constraints.extend(foreign_key_unique_constraints);

    foreign_keys.extend(resolve_model_foreign_keys(
        deferred_model_foreign_keys,
        &columns,
        targets,
        &model,
    )?);
    collected_unique_constraints.extend(resolve_model_unique_constraints(
        deferred_model_unique_constraints,
        &columns,
        &model,
    )?);
    collected_indexes.extend(resolve_model_indexes(
        deferred_model_indexes,
        &columns,
        &model,
    )?);

    let indexes = resolve_indexes(
        collected_indexes,
        &primary_key,
        &collected_unique_constraints,
        &model,
    )?;

    Ok(Model {
        columns,
        foreign_keys,
        indexes,
        primary_key,
        table,
        unique_constraints: collected_unique_constraints,
    })
}

fn validate_relation_namespace(
    models: &[Model],
    seen_table_names: &HashMap<String, String>,
) -> Result<(), ModelCodegenError> {
    let mut constraint_index_names: HashMap<String, String> = HashMap::new();

    for model in models {
        if !model.primary_key.is_empty() {
            register_constraint_index_name(
                seen_table_names,
                &mut constraint_index_names,
                primary_key_index_name(&model.table).map_err(|source| {
                    ModelCodegenError::PrimaryKeyIndexNameTooLong {
                        model: model.table.clone(),
                        source,
                    }
                })?,
                &model.table,
            )?;
        }

        for unique_constraint in &model.unique_constraints {
            register_constraint_index_name(
                seen_table_names,
                &mut constraint_index_names,
                unique_index_name(&model.table, &unique_constraint.columns).map_err(|source| {
                    ModelCodegenError::UniqueIndexNameTooLong {
                        model: model.table.clone(),
                        source,
                    }
                })?,
                &model.table,
            )?;
        }
    }

    let mut index_names: HashMap<String, String> = HashMap::new();

    for model in models {
        for index in &model.indexes {
            register_index_name(
                seen_table_names,
                &constraint_index_names,
                &mut index_names,
                index.name.clone(),
                &model.table,
            )?;
        }
    }

    Ok(())
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

/// # Errors
///
/// Returns `ModelCodegenError` propagated from the work it performs.
pub fn models(attribute_index: &AttributeIndex) -> Result<Vec<Model>, ModelCodegenError> {
    let mut targets: HashMap<CanonicalPath, ForeignKeyTarget> = HashMap::new();
    let mut seen_table_names: HashMap<String, String> = HashMap::new();
    let collected = collect_models(attribute_index, &mut targets, &mut seen_table_names)?;

    let resolved: Vec<Model> = collected
        .into_iter()
        .map(|collected_model| resolve_model(collected_model, &targets))
        .collect::<Result<Vec<Model>, ModelCodegenError>>()?;

    validate_relation_namespace(&resolved, &seen_table_names)?;

    order_by_dependencies(resolved)
}
