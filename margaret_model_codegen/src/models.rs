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
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_matching_attributes::select_matching_attributes;
use margaret_attributes::select_unique_attribute::select_unique_attribute;
use margaret_schema_identifier_naming::index_name::index_name;
use margaret_schema_identifier_naming::primary_key_index_name::primary_key_index_name;
use margaret_schema_identifier_naming::schema_identifier::schema_identifier;
use margaret_schema_identifier_naming::unique_index_name::unique_index_name;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;
use margaret_toposort::topological_order::topological_order;

use crate::collected_model::CollectedModel;
use crate::column_arguments::ColumnArguments;
use crate::deferred_foreign_key::DeferredForeignKey;
use crate::foreign_key_arguments::ForeignKeyArguments;
use crate::foreign_key_target::ForeignKeyTarget;
use crate::foreign_key_target_column::ForeignKeyTargetColumn;
use crate::index_arguments::IndexArguments;
use crate::index_membership::IndexMembership;
use crate::infer_column_type::infer_column_type;
use crate::inferred_column::InferredColumn;
use crate::model::Model;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::option_inner::option_inner;
use crate::positioned_field::PositionedField;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;

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

fn resolve_index_membership(
    arguments: &AttributeArgs,
    model: &str,
) -> Result<IndexMembership, ModelCodegenError> {
    let IndexArguments { name } = IndexArguments::parse(arguments)?;

    match name {
        None => Ok(IndexMembership::Derived),
        Some(name) => {
            if !is_snake_case_identifier(&name) {
                return Err(ModelCodegenError::InvalidIndexName {
                    index: name,
                    model: model.to_string(),
                });
            }

            if let Err(source) = validate_identifier_length(&name) {
                return Err(ModelCodegenError::ExplicitIndexNameTooLong {
                    index: name,
                    model: model.to_string(),
                    source,
                });
            }

            Ok(IndexMembership::Named(name))
        }
    }
}

fn resolve_index_memberships(
    index_attributes: &[&IndexedAttribute],
    field: &IndexedField,
    model: &str,
) -> Result<Vec<IndexMembership>, ModelCodegenError> {
    let mut memberships: Vec<IndexMembership> = Vec::new();
    let mut has_derived = false;
    let mut seen_names: HashSet<String> = HashSet::new();

    for index_attribute in index_attributes {
        let membership = resolve_index_membership(index_attribute.args()?, model)?;

        let repeated = match &membership {
            IndexMembership::Derived => std::mem::replace(&mut has_derived, true),
            IndexMembership::Named(name) => !seen_names.insert(name.clone()),
        };

        if repeated {
            return Err(ModelCodegenError::RepeatedIndexOnColumn {
                field: field_display(field.identifier()),
                model: model.to_string(),
            });
        }

        memberships.push(membership);
    }

    Ok(memberships)
}

fn column_list(columns: &[String]) -> String {
    columns.join(", ")
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
        name,
        primary_key,
        unique,
    }: ColumnArguments,
    indexes: Vec<IndexMembership>,
    PositionedField { field, position }: PositionedField,
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
    let inferred = infer_column_type(field.ty(), model, &column_name)?;

    Ok(ResolvedColumn {
        indexes,
        inferred,
        name: column_name,
        position,
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
    indexes: Vec<IndexMembership>,
    PositionedField { field, position }: PositionedField,
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
        indexes,
        nullable,
        on_delete,
        position,
        rust_type,
        target_path,
        unique,
    })
}

fn collect_models(
    attribute_index: &AttributeIndex,
    targets: &mut HashMap<CanonicalPath, ForeignKeyTarget>,
    seen_table_names: &mut HashMap<String, String>,
) -> Result<Vec<CollectedModel>, ModelCodegenError> {
    let selector = AttributeSelector::from_marker("model");
    let column_selector = AttributeSelector::from_marker("column");
    let foreign_key_selector = AttributeSelector::from_marker("foreign_key");
    let index_selector = AttributeSelector::from_marker("index");
    let mut collected: Vec<CollectedModel> = Vec::new();
    let mut seen_models: HashSet<String> = HashSet::new();

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

        register_table_name(seen_table_names, table.clone(), &model)?;

        let mut scalar_columns: Vec<ResolvedColumn> = Vec::new();
        let mut deferred_foreign_keys: Vec<DeferredForeignKey> = Vec::new();
        let mut seen_columns: HashSet<String> = HashSet::new();

        for (position, field) in item.fields().iter().enumerate() {
            let column_attribute =
                select_unique_attribute(field.attributes(), &column_selector, || model.clone())?;
            let foreign_key_attribute =
                select_unique_attribute(field.attributes(), &foreign_key_selector, || {
                    model.clone()
                })?;
            let index_attributes = select_matching_attributes(field.attributes(), &index_selector);

            if !index_attributes.is_empty() && column_attribute.is_none() {
                return Err(ModelCodegenError::IndexRequiresColumn {
                    field: field_display(field.identifier()),
                    model,
                });
            }

            let indexes = resolve_index_memberships(&index_attributes, field, &model)?;

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
                                indexes,
                                PositionedField { field, position },
                                &model,
                                &mut seen_columns,
                            )?);
                        }
                        Some(foreign_key_attribute) => {
                            deferred_foreign_keys.push(defer_foreign_key(
                                column_arguments,
                                indexes,
                                PositionedField { field, position },
                                foreign_key_attribute.args()?,
                                &model,
                                attribute_index,
                                item,
                            )?);
                        }
                    }
                }
            }
        }

        let primary_key_columns: Vec<&ResolvedColumn> = scalar_columns
            .iter()
            .filter(|column| column.primary_key)
            .collect();

        let target_primary_key: Vec<ForeignKeyTargetColumn> = primary_key_columns
            .iter()
            .map(|column| ForeignKeyTargetColumn {
                column_type: column.inferred.column_type.clone(),
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
            model,
            primary_key,
            scalar_columns,
            seen_columns,
            table,
        });
    }

    Ok(collected)
}

fn resolve_indexes(
    table: &str,
    columns: &[ResolvedColumn],
    model: &str,
) -> Result<Vec<ResolvedIndex>, ModelCodegenError> {
    let mut indexes: Vec<ResolvedIndex> = Vec::new();
    let mut named_groups: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();

    for column in columns {
        for membership in &column.indexes {
            match membership {
                IndexMembership::Derived => {
                    let name = index_name(table, &column.name).map_err(|source| {
                        ModelCodegenError::IndexNameTooLong {
                            model: model.to_string(),
                            source,
                        }
                    })?;

                    indexes.push(ResolvedIndex {
                        columns: vec![column.name.clone()],
                        name,
                    });
                }
                IndexMembership::Named(name) => {
                    named_groups
                        .entry(name.clone())
                        .or_default()
                        .push((column.position, column.name.clone()));
                }
            }
        }
    }

    for (name, mut members) in named_groups {
        members.sort_by_key(|(position, _)| *position);

        indexes.push(ResolvedIndex {
            columns: members
                .into_iter()
                .map(|(_, column_name)| column_name)
                .collect(),
            name,
        });
    }

    indexes.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(indexes)
}

fn validate_index_redundancy(
    indexes: &[ResolvedIndex],
    primary_key: &[String],
    unique_constraints: &[ResolvedUniqueConstraint],
    model: &str,
) -> Result<(), ModelCodegenError> {
    let mut covered_column_lists: HashMap<&[String], &str> = HashMap::new();

    for index in indexes {
        if primary_key.starts_with(&index.columns) {
            return Err(ModelCodegenError::IndexCoveredByPrimaryKey {
                columns: column_list(&index.columns),
                index: index.name.clone(),
                model: model.to_string(),
                primary_key: column_list(primary_key),
            });
        }

        for unique_constraint in unique_constraints {
            if unique_constraint.columns.starts_with(&index.columns) {
                return Err(ModelCodegenError::IndexCoveredByUniqueConstraint {
                    columns: column_list(&index.columns),
                    index: index.name.clone(),
                    model: model.to_string(),
                    unique_constraint: column_list(&unique_constraint.columns),
                });
            }
        }

        if let Some(first) = covered_column_lists.get(index.columns.as_slice()) {
            return Err(ModelCodegenError::DuplicateIndexColumns {
                columns: column_list(&index.columns),
                first: (*first).to_string(),
                model: model.to_string(),
                second: index.name.clone(),
            });
        }

        covered_column_lists.insert(&index.columns, &index.name);
    }

    Ok(())
}

fn resolve_model(
    collected: CollectedModel,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Model, ModelCodegenError> {
    let CollectedModel {
        deferred_foreign_keys,
        model,
        primary_key,
        scalar_columns: mut columns,
        mut seen_columns,
        table,
    } = collected;

    let mut foreign_keys: Vec<ResolvedForeignKey> = Vec::new();

    for deferred in deferred_foreign_keys {
        let DeferredForeignKey {
            field_name,
            indexes,
            nullable,
            on_delete,
            position,
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
            indexes,
            inferred: InferredColumn {
                column_type: referenced.column_type.clone(),
                default: quote!(margaret_model::column_default::ColumnDefault::NotSet),
                nullable,
            },
            name: column_name.clone(),
            position,
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

    let mut unique_constraints: Vec<ResolvedUniqueConstraint> = Vec::new();

    for column in &columns {
        if column.unique {
            unique_constraints.push(ResolvedUniqueConstraint {
                columns: vec![column.name.clone()],
            });
        }
    }

    for foreign_key in &foreign_keys {
        if foreign_key.unique {
            unique_constraints.push(ResolvedUniqueConstraint {
                columns: vec![foreign_key.column.clone()],
            });
        }
    }

    let indexes = resolve_indexes(&table, &columns, &model)?;

    validate_index_redundancy(&indexes, &primary_key, &unique_constraints, &model)?;

    Ok(Model {
        columns,
        foreign_keys,
        indexes,
        primary_key,
        table,
        unique_constraints,
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
                primary_key_index_name(&model.table),
                &model.table,
            )?;
        }

        for unique_constraint in &model.unique_constraints {
            register_constraint_index_name(
                seen_table_names,
                &mut constraint_index_names,
                unique_index_name(&model.table, &unique_constraint.columns),
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
