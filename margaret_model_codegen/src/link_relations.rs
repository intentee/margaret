use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::assembled_model::AssembledModel;
use crate::declared_relation::DeclaredRelation;
use crate::field_value::FieldValue;
use crate::index_kind::IndexKind;
use crate::inverted_key::InvertedKey;
use crate::model::Model;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_field::ModelField;
use crate::model_index::ModelIndex;
use crate::model_relation::ModelRelation;
use crate::relation_kind::RelationKind;

fn is_total(related: &AssembledModel, fields: &[ModelField]) -> bool {
    related
        .indexes
        .all()
        .filter(|index| index.kind != IndexKind::Plain)
        .any(|key| key.fields.iter().all(|field| fields.contains(field)))
}

fn covering_index(
    related: &AssembledModel,
    relation: &DeclaredRelation,
    model: &str,
) -> Result<ModelIndex, ModelCodegenError> {
    let unique_on_key = related.indexes.all().find(|index| {
        index.kind != IndexKind::Plain
            && matches!(index.fields.as_slice(), [only] if only.name == relation.key_field)
    });

    match relation.kind {
        RelationKind::HasOne => {
            unique_on_key
                .cloned()
                .ok_or_else(|| ModelCodegenError::HasOneRequiresUniqueKey {
                    key: relation.key_field.clone(),
                    model: model.to_string(),
                    relation: relation.name.clone(),
                })
        }
        RelationKind::HasMany => {
            if unique_on_key.is_some() {
                return Err(ModelCodegenError::HasManyOverUniqueKey {
                    key: relation.key_field.clone(),
                    model: model.to_string(),
                    relation: relation.name.clone(),
                });
            }

            let covering: Vec<&ModelIndex> = related
                .indexes
                .all()
                .filter(|index| {
                    index
                        .fields
                        .first()
                        .is_some_and(|leading| leading.name == relation.key_field)
                        && index.fields.len() > 1
                        && is_total(related, &index.fields)
                })
                .collect();

            match covering.as_slice() {
                [index] => Ok((*index).clone()),
                [] => Err(ModelCodegenError::HasManyRequiresOrderedIndex {
                    key: relation.key_field.clone(),
                    model: model.to_string(),
                    relation: relation.name.clone(),
                }),
                _ => Err(ModelCodegenError::AmbiguousHasManyIndex {
                    key: relation.key_field.clone(),
                    model: model.to_string(),
                    relation: relation.name.clone(),
                }),
            }
        }
    }
}

fn model_relation(
    relation: &DeclaredRelation,
    parent: &AssembledModel,
    assembled: &HashMap<CanonicalPath, &AssembledModel>,
    model: &str,
) -> Result<ModelRelation, ModelCodegenError> {
    let related = assembled.get(&relation.related).copied().ok_or_else(|| {
        ModelCodegenError::RelatedItemNotAModel {
            model: model.to_string(),
            related: relation.related.to_string(),
            relation: relation.name.clone(),
        }
    })?;
    let key = related
        .fields
        .iter()
        .find(|field| field.name == relation.key_field)
        .ok_or_else(|| ModelCodegenError::RelationKeyFieldNotDeclared {
            key: relation.key_field.clone(),
            model: model.to_string(),
            relation: relation.name.clone(),
        })?;

    match &key.value {
        FieldValue::Key { target, .. } if *target == parent.path => Ok(ModelRelation {
            covering_index: covering_index(related, relation, model)?,
            key: key.clone(),
            kind: relation.kind,
            name: relation.name.clone(),
            related: relation.related.clone(),
        }),
        FieldValue::Key { target, .. } => Err(ModelCodegenError::RelationKeyTargetsAnotherModel {
            key: relation.key_field.clone(),
            model: model.to_string(),
            relation: relation.name.clone(),
            target: target.to_string(),
        }),
        FieldValue::Enum { .. } | FieldValue::Json { .. } | FieldValue::Scalar { .. } => {
            Err(ModelCodegenError::RelationKeyFieldNotAKey {
                key: relation.key_field.clone(),
                model: model.to_string(),
                relation: relation.name.clone(),
            })
        }
    }
}

fn relations(
    parent: &AssembledModel,
    assembled: &HashMap<CanonicalPath, &AssembledModel>,
) -> Result<Vec<ModelRelation>, ModelCodegenError> {
    let model = parent.path.to_string();
    let mut names: HashSet<String> = parent
        .fields
        .iter()
        .filter(|field| matches!(field.value, FieldValue::Key { .. }))
        .map(|field| field.name.clone())
        .collect();
    let mut inverted: HashSet<InvertedKey> = HashSet::new();
    let mut linked: Vec<ModelRelation> = Vec::with_capacity(parent.relations.len());

    for relation in &parent.relations {
        if !names.insert(relation.name.clone()) {
            return Err(ModelCodegenError::DuplicateRelationName {
                model: model.clone(),
                relation: relation.name.clone(),
            });
        }

        if !inverted.insert(InvertedKey {
            key_field: relation.key_field.clone(),
            related: relation.related.clone(),
        }) {
            return Err(ModelCodegenError::DuplicateInverseRelation {
                key: relation.key_field.clone(),
                model: model.clone(),
                related: relation.related.to_string(),
            });
        }

        linked.push(model_relation(relation, parent, assembled, &model)?);
    }

    Ok(linked)
}

pub(crate) fn link_relations(
    assembled: &[AssembledModel],
) -> Result<Vec<Model>, ModelCodegenError> {
    let mut by_path: HashMap<CanonicalPath, &AssembledModel> = HashMap::new();

    for model in assembled {
        by_path.insert(model.path.clone(), model);
    }

    assembled
        .iter()
        .map(|model| {
            Ok(Model {
                fields: model.fields.clone(),
                foreign_keys: model.foreign_keys.clone(),
                indexes: model.indexes.clone(),
                module: model.module.clone(),
                namespace: model.namespace,
                path: model.path.clone(),
                relations: relations(model, &by_path)?,
                table: model.table.clone(),
            })
        })
        .collect()
}
