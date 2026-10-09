use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::is_snake_case_name::is_snake_case_name;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::select_framework_attributes::select_framework_attributes;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;

use crate::collect_field::collect_field;
use crate::collected_field::CollectedField;
use crate::collected_field_shape::CollectedFieldShape;
use crate::collected_model::CollectedModel;
use crate::declared_index::DeclaredIndex;
use crate::declared_relation::DeclaredRelation;
use crate::enum_variants::EnumVariants;
use crate::key_column::KeyColumn;
use crate::model_arguments::ModelArguments;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_index_arguments::ModelIndexArguments;
use crate::model_primary_key_arguments::ModelPrimaryKeyArguments;
use crate::model_unique_arguments::ModelUniqueArguments;
use crate::primary_key_part::PrimaryKeyPart;
use crate::primary_key_source::PrimaryKeySource;
use crate::relation_arguments::RelationArguments;
use crate::relation_kind::RelationKind;

fn table_name(arguments: ModelArguments, model: &str) -> Result<String, ModelCodegenError> {
    let ModelArguments { table } = arguments;

    if !is_snake_case_name(&table) {
        return Err(ModelCodegenError::InvalidTableName {
            model: model.to_string(),
            table,
        });
    }

    validate_identifier_length(&table).map_err(|source| ModelCodegenError::TableNameTooLong {
        model: model.to_string(),
        source,
    })?;

    Ok(table)
}

fn declared_fields_exist(
    fields: &[String],
    collected: &[CollectedField],
    declaration: FrameworkAttribute,
    model: &str,
) -> Result<(), ModelCodegenError> {
    match fields
        .iter()
        .find(|declared| !collected.iter().any(|field| &field.name == *declared))
    {
        Some(missing) => Err(ModelCodegenError::ModelDeclarationFieldNotDeclared {
            declaration: declaration.name().to_string(),
            field: missing.clone(),
            model: model.to_string(),
        }),
        None => Ok(()),
    }
}

fn primary_key_part(
    name: &str,
    fields: &[CollectedField],
    model: &str,
) -> Result<PrimaryKeyPart, ModelCodegenError> {
    let field = fields
        .iter()
        .find(|field| field.name == name)
        .ok_or_else(|| ModelCodegenError::ModelDeclarationFieldNotDeclared {
            declaration: FrameworkAttribute::PrimaryKey.name().to_string(),
            field: name.to_string(),
            model: model.to_string(),
        })?;

    if field.nullable {
        return Err(ModelCodegenError::NullablePrimaryKeyField {
            field: name.to_string(),
            model: model.to_string(),
        });
    }

    Ok(PrimaryKeyPart {
        field: name.to_string(),
        source: match &field.shape {
            CollectedFieldShape::Column(column) => PrimaryKeySource::Column(KeyColumn {
                column_type: column.column_type,
                name: column.name.clone(),
            }),
            CollectedFieldShape::Key { target, .. } => PrimaryKeySource::Key {
                target: target.clone(),
            },
        },
    })
}

fn primary_key_names(
    item: &IndexedItem,
    fields: &[CollectedField],
    model: &str,
) -> Result<Vec<String>, ModelCodegenError> {
    let flagged: Vec<String> = fields
        .iter()
        .filter(|field| field.primary_key)
        .map(|field| field.name.clone())
        .collect();

    match AttributeQuery::new(item).find_framework(FrameworkAttribute::PrimaryKey) {
        None => match flagged.as_slice() {
            [] => Err(ModelCodegenError::ModelRequiresPrimaryKey {
                model: model.to_string(),
            }),
            [_] => Ok(flagged),
            _ => Err(
                ModelCodegenError::CompositePrimaryKeyRequiresModelDeclaration {
                    model: model.to_string(),
                },
            ),
        },
        Some(_) if !flagged.is_empty() => {
            Err(ModelCodegenError::ConflictingPrimaryKeyDeclarations {
                model: model.to_string(),
            })
        }
        Some(declaration) => {
            let ModelPrimaryKeyArguments { fields: declared } =
                ModelPrimaryKeyArguments::parse(declaration.args()?, model)?;

            Ok(declared)
        }
    }
}

fn primary_key(
    item: &IndexedItem,
    fields: &[CollectedField],
    model: &str,
) -> Result<Vec<PrimaryKeyPart>, ModelCodegenError> {
    primary_key_names(item, fields, model)?
        .iter()
        .map(|name| primary_key_part(name, fields, model))
        .collect()
}

fn uniques(
    item: &IndexedItem,
    fields: &[CollectedField],
    model: &str,
) -> Result<Vec<Vec<String>>, ModelCodegenError> {
    let mut declared: Vec<Vec<String>> = Vec::new();
    let mut seen: HashSet<Vec<String>> = HashSet::new();

    for attribute in select_framework_attributes(item.attributes(), FrameworkAttribute::Unique) {
        let ModelUniqueArguments { fields: unique } =
            ModelUniqueArguments::parse(attribute.args()?, model)?;

        declared_fields_exist(&unique, fields, FrameworkAttribute::Unique, model)?;

        if !seen.insert(unique.clone()) {
            return Err(ModelCodegenError::DuplicateModelUniqueConstraint {
                fields: unique.join(", "),
                model: model.to_string(),
            });
        }

        declared.push(unique);
    }

    Ok(declared)
}

fn indexes(
    item: &IndexedItem,
    fields: &[CollectedField],
    model: &str,
) -> Result<Vec<DeclaredIndex>, ModelCodegenError> {
    let mut declared: Vec<DeclaredIndex> = Vec::new();

    for attribute in select_framework_attributes(item.attributes(), FrameworkAttribute::Index) {
        let ModelIndexArguments {
            fields: indexed,
            name,
        } = ModelIndexArguments::parse(attribute.args()?, model)?;

        declared_fields_exist(&indexed, fields, FrameworkAttribute::Index, model)?;
        declared.push(DeclaredIndex {
            fields: indexed,
            name,
        });
    }

    Ok(declared)
}

fn relations(
    index: &AttributeIndex,
    item: &IndexedItem,
    model: &str,
) -> Result<Vec<DeclaredRelation>, ModelCodegenError> {
    let mut declared: Vec<DeclaredRelation> = Vec::new();

    for kind in [RelationKind::HasMany, RelationKind::HasOne] {
        let declaration = kind.declaration();

        for attribute in select_framework_attributes(item.attributes(), declaration) {
            let RelationArguments {
                key_field,
                name,
                related,
            } = RelationArguments::parse(attribute.args()?, declaration, index, item, model)?;

            declared.push(DeclaredRelation {
                key_field,
                kind,
                name,
                related,
            });
        }
    }

    Ok(declared)
}

pub(crate) fn collect_model(
    index: &AttributeIndex,
    matched: &MatchedAttribute,
    enum_variants: &mut EnumVariants,
) -> Result<CollectedModel, ModelCodegenError> {
    let item = matched.item();
    let model = item.canonical_path().to_string();

    let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
        return Err(ModelCodegenError::ModelNotAStruct { model });
    };

    if !select_framework_attributes(item.attributes(), FrameworkAttribute::ForeignKey).is_empty() {
        return Err(ModelCodegenError::ForeignKeyIsNotAModelAttribute { model });
    }

    let table = table_name(ModelArguments::parse(matched.args()?, &model)?, &model)?;
    let fields = item
        .fields()
        .iter()
        .map(|field| collect_field(index, item, field, enum_variants, &table, &model))
        .collect::<Result<Vec<CollectedField>, ModelCodegenError>>()?;

    Ok(CollectedModel {
        indexes: indexes(item, &fields, &model)?,
        module: identifier.field().to_string(),
        path: item.canonical_path().clone(),
        primary_key: primary_key(item, &fields, &model)?,
        relations: relations(index, item, &model)?,
        uniques: uniques(item, &fields, &model)?,
        fields,
        table,
    })
}
