use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;
use syn::PathSegment;
use syn::Type;
use syn::TypePath;
use syn::punctuated::Punctuated;
use syn::token::PathSep;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::format_path::format_path;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::select_matching_attributes::select_matching_attributes;
use margaret_attributes::select_unique_attribute::select_unique_attribute;

use margaret_schema_identifier_naming::index_name::index_name;
use margaret_schema_identifier_naming::primary_key_index_name::primary_key_index_name;
use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;
use margaret_schema_identifier_naming::unique_index_name::unique_index_name;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;

use margaret_toposort::topological_order::topological_order;

use crate::collected_model::CollectedModel;
use crate::column_arguments::ColumnArguments;
use crate::column_id::ColumnId;
use crate::deferred_foreign_key_member::DeferredForeignKeyMember;
use crate::foreign_key_arguments::ForeignKeyArguments;
use crate::foreign_key_target::ForeignKeyTarget;
use crate::foreign_key_target_column::ForeignKeyTargetColumn;
use crate::index_arguments::IndexArguments;
use crate::index_membership::IndexMembership;
use crate::infer_column_type::infer_column_type;
use crate::model::Model;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::object_kind::ObjectKind;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;

struct ForeignKeyGroup {
    members: Vec<DeferredForeignKeyMember>,
    name: SchemaIdentifier,
}

struct IndexColumn {
    id: ColumnId,
    name: String,
    position: usize,
}

struct ModelObjectNames {
    primary_key_index: Option<SchemaIdentifier>,
    unique_indexes: Vec<SchemaIdentifier>,
}

struct RelationEntry {
    kind: ObjectKind,
    table: String,
}

struct ResolvedReference {
    referenced_field: String,
    reference_display: String,
    target_path: CanonicalPath,
}

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
    arguments: &margaret_attributes::attribute_args::AttributeArgs,
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

            match SchemaIdentifier::new(name.clone()) {
                Ok(identifier) => Ok(IndexMembership::Named(identifier)),
                Err(source) => Err(ModelCodegenError::ExplicitIndexNameTooLong {
                    index: name,
                    model: model.to_string(),
                    source,
                }),
            }
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
    let mut seen_names: HashSet<SchemaIdentifier> = HashSet::new();

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

fn is_leading_prefix(index: &[ColumnId], key: &[ColumnId]) -> bool {
    index.len() <= key.len() && index == &key[..index.len()]
}

fn ensure_index_not_redundant(
    index_ids: &[ColumnId],
    index_columns: &[String],
    primary_key: &[ColumnId],
    unique_keys: &[Vec<ColumnId>],
    model: &str,
) -> Result<(), ModelCodegenError> {
    if is_leading_prefix(index_ids, primary_key) {
        return Err(ModelCodegenError::RedundantIndexOnPrimaryKeyPrefix {
            columns: index_columns.join(", "),
            model: model.to_string(),
        });
    }

    for unique in unique_keys {
        if is_leading_prefix(index_ids, unique) {
            return Err(ModelCodegenError::RedundantIndexOnUniquePrefix {
                columns: index_columns.join(", "),
                model: model.to_string(),
            });
        }
    }

    Ok(())
}

fn resolve_indexes(
    table: &SchemaIdentifier,
    columns: &[ResolvedColumn],
    primary_key: &[ColumnId],
    unique_keys: &[Vec<ColumnId>],
    model: &str,
) -> Result<Vec<ResolvedIndex>, ModelCodegenError> {
    let mut indexes: Vec<ResolvedIndex> = Vec::new();
    let mut named_groups: BTreeMap<SchemaIdentifier, Vec<IndexColumn>> = BTreeMap::new();

    for column in columns {
        let column_id = ColumnId::new(column.position);

        for membership in &column.indexes {
            match membership {
                IndexMembership::Derived => {
                    ensure_index_not_redundant(
                        &[column_id],
                        std::slice::from_ref(&column.name),
                        primary_key,
                        unique_keys,
                        model,
                    )?;

                    let name = index_name(table.as_str(), &column.name).map_err(|source| {
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
                    named_groups.entry(name.clone()).or_default().push(IndexColumn {
                        id: column_id,
                        name: column.name.clone(),
                        position: column.position,
                    });
                }
            }
        }
    }

    for (name, mut members) in named_groups {
        members.sort_by_key(|member| member.position);

        let ids: Vec<ColumnId> = members.iter().map(|member| member.id).collect();
        let column_names: Vec<String> = members.into_iter().map(|member| member.name).collect();

        ensure_index_not_redundant(&ids, &column_names, primary_key, unique_keys, model)?;

        indexes.push(ResolvedIndex {
            columns: column_names,
            name,
        });
    }

    indexes.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(indexes)
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

fn canonicalize_reference(
    attribute_index: &AttributeIndex,
    item: &IndexedItem,
    references: &Path,
    field: &str,
    model: &str,
) -> Result<ResolvedReference, ModelCodegenError> {
    let reference_display = format_path(references);
    let field_count = references.segments.len();

    let referenced_field = match references.segments.last() {
        Some(segment) if field_count >= 2 => segment.ident.to_string(),
        _ => {
            return Err(ModelCodegenError::ForeignKeyReferenceMissingField {
                field: field.to_string(),
                model: model.to_string(),
                reference: reference_display,
            });
        }
    };

    let prefix_segments: Punctuated<PathSegment, PathSep> = references
        .segments
        .iter()
        .take(field_count - 1)
        .cloned()
        .collect();

    let prefix_type = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: references.leading_colon,
            segments: prefix_segments,
        },
    });

    let target_path = attribute_index.resolve_item_type(item, &prefix_type).ok_or_else(|| {
        ModelCodegenError::ForeignKeyTargetNotAModel {
            field: field.to_string(),
            model: model.to_string(),
            reference: reference_display.clone(),
        }
    })?;

    Ok(ResolvedReference {
        reference_display,
        referenced_field,
        target_path,
    })
}

fn resolve_scalar_column(
    ColumnArguments {
        name,
        primary_key,
        unique,
    }: ColumnArguments,
    indexes: Vec<IndexMembership>,
    field: &IndexedField,
    position: usize,
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

        let table = match SchemaIdentifier::new(table) {
            Ok(table) => table,
            Err(source) => return Err(ModelCodegenError::TableNameTooLong { model, source }),
        };

        register_table_name(seen_table_names, table.as_str().to_string(), &model)?;

        let mut columns: Vec<ResolvedColumn> = Vec::new();
        let mut deferred_foreign_key_members: Vec<DeferredForeignKeyMember> = Vec::new();
        let mut columns_by_field: HashMap<String, ForeignKeyTargetColumn> = HashMap::new();
        let mut primary_key: Vec<ColumnId> = Vec::new();
        let mut unique_keys: Vec<Vec<ColumnId>> = Vec::new();
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

            let column_attribute = match (column_attribute, &foreign_key_attribute) {
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
                (Some(column_attribute), _) => column_attribute,
            };

            let column_arguments = ColumnArguments::parse(column_attribute.args()?)?;
            let column = resolve_scalar_column(
                column_arguments,
                indexes,
                field,
                position,
                &model,
                &mut seen_columns,
            )?;
            let column_id = ColumnId::new(position);

            if column.primary_key {
                primary_key.push(column_id);
            }

            if column.unique {
                unique_keys.push(vec![column_id]);
            }

            if let FieldIdentifier::Named(field_name) = field.identifier() {
                columns_by_field.insert(
                    field_name.clone(),
                    ForeignKeyTargetColumn {
                        column_type: column.inferred.column_type,
                        id: column_id,
                        name: column.name.clone(),
                    },
                );
            }

            if let Some(foreign_key_attribute) = foreign_key_attribute {
                let display = field_display(field.identifier());
                let ForeignKeyArguments {
                    name,
                    on_delete,
                    references,
                } = ForeignKeyArguments::parse(foreign_key_attribute.args()?, &model, &display)?;

                if !is_snake_case_identifier(&name) {
                    return Err(ModelCodegenError::InvalidForeignKeyName {
                        field: display,
                        model,
                        name,
                    });
                }

                let fk_name = match SchemaIdentifier::new(name.clone()) {
                    Ok(fk_name) => fk_name,
                    Err(source) => {
                        return Err(ModelCodegenError::ForeignKeyNameTooLong {
                            field: display,
                            model,
                            name,
                            source,
                        });
                    }
                };

                let ResolvedReference {
                    reference_display,
                    referenced_field,
                    target_path,
                } = canonicalize_reference(attribute_index, item, &references, &display, &model)?;

                deferred_foreign_key_members.push(DeferredForeignKeyMember {
                    field_display: display,
                    local_column: column.name.clone(),
                    local_type: column.inferred.column_type,
                    name: fk_name,
                    on_delete,
                    reference_display,
                    referenced_field,
                    target_path,
                });
            }

            columns.push(column);
        }

        targets.insert(
            item.canonical_path().clone(),
            ForeignKeyTarget {
                columns_by_field,
                primary_key: primary_key.clone(),
                table: table.as_str().to_string(),
                unique_keys: unique_keys.clone(),
            },
        );

        collected.push(CollectedModel {
            columns,
            deferred_foreign_key_members,
            model,
            primary_key,
            table,
            unique_keys,
        });
    }

    Ok(collected)
}

fn references_match_key(references: &[ColumnId], key: &[ColumnId]) -> bool {
    references.len() == key.len()
        && references.iter().collect::<HashSet<&ColumnId>>() == key.iter().collect::<HashSet<&ColumnId>>()
}

fn group_foreign_key_members(members: Vec<DeferredForeignKeyMember>) -> Vec<ForeignKeyGroup> {
    let mut groups: Vec<ForeignKeyGroup> = Vec::new();

    for member in members {
        match groups.iter_mut().find(|group| group.name == member.name) {
            Some(group) => group.members.push(member),
            None => groups.push(ForeignKeyGroup {
                name: member.name.clone(),
                members: vec![member],
            }),
        }
    }

    groups
}

fn resolve_foreign_key_group(
    ForeignKeyGroup { members, name }: ForeignKeyGroup,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
    model: &str,
) -> Result<ResolvedForeignKey, ModelCodegenError> {
    let target_path = &members[0].target_path;

    for member in &members {
        if &member.target_path != target_path {
            return Err(ModelCodegenError::ForeignKeyAmbiguousTarget {
                model: model.to_string(),
                name: name.as_str().to_string(),
            });
        }
    }

    let target = targets.get(target_path).ok_or_else(|| {
        ModelCodegenError::ForeignKeyTargetNotAModel {
            field: members[0].field_display.clone(),
            model: model.to_string(),
            reference: members[0].reference_display.clone(),
        }
    })?;

    let mut columns: Vec<String> = Vec::new();
    let mut references_columns: Vec<String> = Vec::new();
    let mut referenced_ids: Vec<ColumnId> = Vec::new();

    for member in &members {
        let referenced = target.columns_by_field.get(&member.referenced_field).ok_or_else(|| {
            ModelCodegenError::ForeignKeyUnknownReferencedField {
                field: member.field_display.clone(),
                model: model.to_string(),
                reference: member.reference_display.clone(),
                target: target.table.clone(),
            }
        })?;

        if member.local_type != referenced.column_type {
            return Err(ModelCodegenError::ForeignKeyColumnTypeMismatch {
                field: member.field_display.clone(),
                local_type: member.local_type,
                model: model.to_string(),
                referenced_type: referenced.column_type,
            });
        }

        if referenced_ids.contains(&referenced.id) {
            return Err(ModelCodegenError::ForeignKeyDuplicateReferencedColumn {
                model: model.to_string(),
                name: name.as_str().to_string(),
            });
        }

        columns.push(member.local_column.clone());
        references_columns.push(referenced.name.clone());
        referenced_ids.push(referenced.id);
    }

    let on_delete = members[0].on_delete;

    for member in &members {
        if member.on_delete != on_delete {
            return Err(ModelCodegenError::InconsistentForeignKeyOnDelete {
                model: model.to_string(),
                name: name.as_str().to_string(),
            });
        }
    }

    let matches_key = references_match_key(&referenced_ids, &target.primary_key)
        || target
            .unique_keys
            .iter()
            .any(|key| references_match_key(&referenced_ids, key));

    if !matches_key {
        return Err(ModelCodegenError::ForeignKeyReferencedColumnsNotAKey {
            model: model.to_string(),
            name: name.as_str().to_string(),
            target: target.table.clone(),
        });
    }

    Ok(ResolvedForeignKey {
        columns,
        name,
        on_delete,
        references_columns,
        references_table: target.table.clone(),
    })
}

fn resolve_foreign_keys(
    members: Vec<DeferredForeignKeyMember>,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
    model: &str,
) -> Result<Vec<ResolvedForeignKey>, ModelCodegenError> {
    let mut foreign_keys: Vec<ResolvedForeignKey> = group_foreign_key_members(members)
        .into_iter()
        .map(|group| resolve_foreign_key_group(group, targets, model))
        .collect::<Result<Vec<ResolvedForeignKey>, ModelCodegenError>>()?;

    foreign_keys.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(foreign_keys)
}

fn resolve_model(
    collected: CollectedModel,
    targets: &HashMap<CanonicalPath, ForeignKeyTarget>,
) -> Result<Model, ModelCodegenError> {
    let CollectedModel {
        columns,
        deferred_foreign_key_members,
        model,
        primary_key,
        table,
        unique_keys,
    } = collected;

    let foreign_keys = resolve_foreign_keys(deferred_foreign_key_members, targets, &model)?;

    let mut unique_constraints: Vec<ResolvedUniqueConstraint> = Vec::new();

    for column in &columns {
        if column.unique {
            unique_constraints.push(ResolvedUniqueConstraint {
                columns: vec![column.name.clone()],
            });
        }
    }

    let indexes = resolve_indexes(&table, &columns, &primary_key, &unique_keys, &model)?;

    let primary_key_columns: Vec<String> = columns
        .iter()
        .filter(|column| column.primary_key)
        .map(|column| column.name.clone())
        .collect();

    Ok(Model {
        columns,
        foreign_keys,
        indexes,
        primary_key: primary_key_columns,
        table,
        unique_constraints,
    })
}

fn register_relation_name(
    relation: &mut HashMap<SchemaIdentifier, RelationEntry>,
    name: SchemaIdentifier,
    kind: ObjectKind,
    table: &str,
) -> Result<(), ModelCodegenError> {
    if let Some(existing) = relation.get(&name) {
        return Err(ModelCodegenError::RelationNameCollision {
            first_kind: kind,
            first_table: table.to_string(),
            name: name.as_str().to_string(),
            second_kind: existing.kind,
            second_table: existing.table.clone(),
        });
    }

    relation.insert(
        name,
        RelationEntry {
            kind,
            table: table.to_string(),
        },
    );

    Ok(())
}

fn register_constraint_name(
    constraint_names: &mut HashMap<SchemaIdentifier, ObjectKind>,
    name: SchemaIdentifier,
    kind: ObjectKind,
    table: &str,
) -> Result<(), ModelCodegenError> {
    if let Some(existing_kind) = constraint_names.get(&name) {
        return Err(ModelCodegenError::ConstraintNameCollision {
            first_kind: kind,
            name: name.as_str().to_string(),
            second_kind: *existing_kind,
            table: table.to_string(),
        });
    }

    constraint_names.insert(name, kind);

    Ok(())
}

fn model_object_names(model: &Model) -> Result<ModelObjectNames, ModelCodegenError> {
    let primary_key_index = if model.primary_key.is_empty() {
        None
    } else {
        Some(
            primary_key_index_name(model.table.as_str()).map_err(|source| {
                ModelCodegenError::PrimaryKeyIndexNameTooLong {
                    source,
                    table: model.table.as_str().to_string(),
                }
            })?,
        )
    };

    let unique_indexes = model
        .unique_constraints
        .iter()
        .map(|unique_constraint| {
            unique_index_name(model.table.as_str(), &unique_constraint.columns).map_err(|source| {
                ModelCodegenError::UniqueIndexNameTooLong {
                    source,
                    table: model.table.as_str().to_string(),
                }
            })
        })
        .collect::<Result<Vec<SchemaIdentifier>, ModelCodegenError>>()?;

    Ok(ModelObjectNames {
        primary_key_index,
        unique_indexes,
    })
}

fn validate_relation_namespace(
    models: &[Model],
    object_names: &[ModelObjectNames],
) -> Result<(), ModelCodegenError> {
    let mut relation: HashMap<SchemaIdentifier, RelationEntry> = HashMap::new();

    for (model, names) in models.iter().zip(object_names) {
        register_relation_name(
            &mut relation,
            model.table.clone(),
            ObjectKind::Table,
            model.table.as_str(),
        )?;

        if let Some(primary_key_index) = &names.primary_key_index {
            register_relation_name(
                &mut relation,
                primary_key_index.clone(),
                ObjectKind::PrimaryKeyIndex,
                model.table.as_str(),
            )?;
        }

        for unique_index in &names.unique_indexes {
            register_relation_name(
                &mut relation,
                unique_index.clone(),
                ObjectKind::UniqueIndex,
                model.table.as_str(),
            )?;
        }

        for index in &model.indexes {
            register_relation_name(
                &mut relation,
                index.name.clone(),
                ObjectKind::Index,
                model.table.as_str(),
            )?;
        }
    }

    Ok(())
}

fn validate_constraint_namespace(
    model: &Model,
    names: &ModelObjectNames,
) -> Result<(), ModelCodegenError> {
    let mut constraint_names: HashMap<SchemaIdentifier, ObjectKind> = HashMap::new();

    if let Some(primary_key_index) = &names.primary_key_index {
        constraint_names.insert(primary_key_index.clone(), ObjectKind::PrimaryKeyConstraint);
    }

    for unique_index in &names.unique_indexes {
        constraint_names.insert(unique_index.clone(), ObjectKind::UniqueConstraint);
    }

    for foreign_key in &model.foreign_keys {
        register_constraint_name(
            &mut constraint_names,
            foreign_key.name.clone(),
            ObjectKind::ForeignKeyConstraint,
            model.table.as_str(),
        )?;
    }

    Ok(())
}

fn validate_namespaces(models: &[Model]) -> Result<(), ModelCodegenError> {
    let object_names = models
        .iter()
        .map(model_object_names)
        .collect::<Result<Vec<ModelObjectNames>, ModelCodegenError>>()?;

    validate_relation_namespace(models, &object_names)?;

    for (model, names) in models.iter().zip(&object_names) {
        validate_constraint_namespace(model, names)?;
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
                .filter(|referenced_table| referenced_table != model.table.as_str())
                .collect();

            (model.table.as_str().to_string(), references)
        })
        .collect();

    let order =
        topological_order(&dependencies).map_err(|cycle| ModelCodegenError::ForeignKeyCycle {
            path: cycle.path.join(" -> "),
        })?;

    let mut by_table: HashMap<String, Model> = resolved
        .into_iter()
        .map(|model| (model.table.as_str().to_string(), model))
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

    validate_namespaces(&resolved)?;

    order_by_dependencies(resolved)
}
