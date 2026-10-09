use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::field_identifier::FieldIdentifier;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_field::IndexedField;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_model_codegen::field_value::FieldValue;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::relation_kind::RelationKind;
use margaret_sql::maximum_limit::MAXIMUM_LIMIT;

use crate::active_record_codegen_error::ActiveRecordCodegenError;
use crate::loaded_target::LoadedTarget;
use crate::loaded_type::LoadedType;
use crate::loaded_wrapper::LoadedWrapper;
use crate::relation_arguments::RelationArguments;
use crate::shape_declaration::ShapeDeclaration;
use crate::shape_relation::ShapeRelation;
use crate::shape_relation_kind::ShapeRelationKind;

struct DeclaredShape<'index, 'model> {
    item: &'index IndexedItem,
    model: &'model Model,
    module: String,
}

struct ExpectedRelation<'model> {
    kind: ShapeRelationKind<'model>,
    related: &'model CanonicalPath,
    wrapper: LoadedWrapper,
}

fn declared_shape<'index, 'model>(
    index: &'index AttributeIndex,
    item: &'index IndexedItem,
    arguments: &AttributeArgs,
    models: &'model [Model],
) -> Result<DeclaredShape<'index, 'model>, ActiveRecordCodegenError> {
    let shape = item.canonical_path().to_string();
    let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
        return Err(ActiveRecordCodegenError::EagerLoadNotAStruct { shape });
    };
    let named = arguments.interpret(|reader| {
        reader
            .take_path("model")?
            .ok_or_else(|| ActiveRecordCodegenError::EagerLoadRequiresModel {
                shape: shape.clone(),
            })
    })?;
    let model = index
        .resolve_item_path(item, &named)
        .and_then(|resolved| models.iter().find(|model| model.path == resolved))
        .ok_or_else(|| ActiveRecordCodegenError::EagerLoadModelNotAModel {
            model: format_path(&named),
            shape: shape.clone(),
        })?;

    Ok(DeclaredShape {
        item,
        model,
        module: identifier.field().to_string(),
    })
}

fn expected_relation<'model>(
    model: &'model Model,
    RelationArguments { limit, relation }: &RelationArguments,
    shape: &str,
) -> Result<ExpectedRelation<'model>, ActiveRecordCodegenError> {
    let single = |expected: ExpectedRelation<'model>| match limit {
        Some(_) => Err(ActiveRecordCodegenError::LimitOnSingleRelation {
            relation: relation.clone(),
            shape: shape.to_string(),
        }),
        None => Ok(expected),
    };

    if let Some(key) = model.fields.iter().find(|field| &field.name == relation)
        && let FieldValue::Key { target, .. } = &key.value
    {
        return single(ExpectedRelation {
            kind: ShapeRelationKind::BelongsTo { key },
            related: target,
            wrapper: if key.nullable {
                LoadedWrapper::Optional
            } else {
                LoadedWrapper::Bare
            },
        });
    }

    let declared = model
        .relations
        .iter()
        .find(|declared| &declared.name == relation)
        .ok_or_else(|| ActiveRecordCodegenError::UnknownRelation {
            model: model.path.to_string(),
            relation: relation.clone(),
            shape: shape.to_string(),
        })?;

    match declared.kind {
        RelationKind::HasOne => single(ExpectedRelation {
            kind: ShapeRelationKind::HasOne { relation: declared },
            related: &declared.related,
            wrapper: LoadedWrapper::Optional,
        }),
        RelationKind::HasMany => match limit {
            None => Err(ActiveRecordCodegenError::HasManyRelationRequiresLimit {
                relation: relation.clone(),
                shape: shape.to_string(),
            }),
            Some(0) => Err(ActiveRecordCodegenError::RelationLimitMustBePositive {
                relation: relation.clone(),
                shape: shape.to_string(),
            }),
            Some(limit) if *limit as u64 > MAXIMUM_LIMIT => {
                Err(ActiveRecordCodegenError::RelationLimitExceedsMaximum {
                    limit: *limit,
                    relation: relation.clone(),
                    shape: shape.to_string(),
                })
            }
            Some(limit) => Ok(ExpectedRelation {
                kind: ShapeRelationKind::HasMany {
                    limit: *limit,
                    relation: declared,
                },
                related: &declared.related,
                wrapper: LoadedWrapper::Children,
            }),
        },
    }
}

fn shape_relation<'model>(
    index: &AttributeIndex,
    declared: &DeclaredShape<'_, 'model>,
    field: &IndexedField,
    relation_attribute: &IndexedAttribute,
    shape_models: &HashMap<CanonicalPath, &'model Model>,
) -> Result<ShapeRelation<'model>, ActiveRecordCodegenError> {
    let shape = declared.item.canonical_path().to_string();
    let field_name = field.identifier().to_string();
    let arguments = RelationArguments::parse(relation_attribute.args()?, &shape, &field_name)?;
    let ExpectedRelation {
        kind,
        related,
        wrapper,
    } = expected_relation(declared.model, &arguments, &shape)?;
    let mismatch = || ActiveRecordCodegenError::RelationTypeMismatch {
        expected: related.to_string(),
        field: field_name.clone(),
        found: field.ty().to_token_stream().to_string(),
        relation: arguments.relation.clone(),
        shape: shape.clone(),
    };
    let loaded = LoadedType::of(index, declared.item, field.ty()).ok_or_else(mismatch)?;
    let target = if loaded.loaded == *related {
        LoadedTarget::Model
    } else if shape_models
        .get(&loaded.loaded)
        .is_some_and(|model| model.path == *related)
    {
        LoadedTarget::Shape
    } else {
        return Err(mismatch());
    };

    if loaded.wrapper != wrapper {
        return Err(mismatch());
    }

    Ok(ShapeRelation {
        field: field_name,
        kind,
        loaded: loaded.loaded,
        relation: arguments.relation,
        target,
        wrapper,
    })
}

fn shape_declaration<'model>(
    index: &AttributeIndex,
    declared: DeclaredShape<'_, 'model>,
    shape_models: &HashMap<CanonicalPath, &'model Model>,
) -> Result<ShapeDeclaration<'model>, ActiveRecordCodegenError> {
    let shape = declared.item.canonical_path().to_string();
    let mut bases: Vec<String> = Vec::new();
    let mut relations: Vec<ShapeRelation<'model>> = Vec::new();
    let mut relation_names: HashSet<String> = HashSet::new();

    for field in declared.item.fields() {
        let field_name = match field.identifier() {
            FieldIdentifier::Named(name) => name.clone(),
            FieldIdentifier::Positional(position) => {
                return Err(ActiveRecordCodegenError::ShapeRequiresNamedFields {
                    position: *position,
                    shape,
                });
            }
        };
        let is_base = field
            .framework_attribute(FrameworkAttribute::Base)
            .is_some();
        let relation_attribute = field.framework_attribute(FrameworkAttribute::Relation);

        if is_base && relation_attribute.is_some() {
            return Err(ActiveRecordCodegenError::ShapeFieldMarkedTwice {
                field: field_name,
                shape,
            });
        }

        if is_base {
            let found = field.ty().to_token_stream().to_string();
            let holds_model =
                LoadedType::of(index, declared.item, field.ty()).is_some_and(|loaded| {
                    loaded.wrapper == LoadedWrapper::Bare && loaded.loaded == declared.model.path
                });

            if !holds_model {
                return Err(ActiveRecordCodegenError::BaseTypeMismatch {
                    field: field_name,
                    found,
                    model: declared.model.path.to_string(),
                    shape,
                });
            }

            bases.push(field_name);
            continue;
        }

        let Some(relation_attribute) = relation_attribute else {
            return Err(ActiveRecordCodegenError::ShapeFieldUnmarked {
                field: field_name,
                shape,
            });
        };
        let relation = shape_relation(index, &declared, field, relation_attribute, shape_models)?;

        if !relation_names.insert(relation.relation.clone()) {
            return Err(ActiveRecordCodegenError::DuplicateShapeRelation {
                relation: relation.relation,
                shape,
            });
        }

        relations.push(relation);
    }

    match bases.as_slice() {
        [base] => Ok(ShapeDeclaration {
            base: base.clone(),
            model: declared.model,
            module: declared.module,
            path: declared.item.canonical_path().clone(),
            relations,
        }),
        _ => Err(ActiveRecordCodegenError::ShapeBaseCount {
            count: bases.len(),
            shape,
        }),
    }
}

/// # Errors
///
/// Returns `ActiveRecordCodegenError` when an `#[eager_load]` declaration does not match the
/// models it loads.
pub fn collect_shapes<'model>(
    index: &AttributeIndex,
    models: &'model [Model],
) -> Result<Vec<ShapeDeclaration<'model>>, ActiveRecordCodegenError> {
    let mut shape_models: HashMap<CanonicalPath, &'model Model> = HashMap::new();
    let mut declared: Vec<DeclaredShape<'_, 'model>> = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::EagerLoad) {
        let shape = declared_shape(index, matched.item(), matched.args()?, models)?;

        shape_models.insert(shape.item.canonical_path().clone(), shape.model);
        declared.push(shape);
    }

    declared
        .into_iter()
        .map(|shape| shape_declaration(index, shape, &shape_models))
        .collect()
}
