use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_schema_identifier_naming::schema_identifier::schema_identifier;

use crate::collected_model::CollectedModel;
use crate::key_column::KeyColumn;
use crate::key_reference::KeyReference;
use crate::model_codegen_error::ModelCodegenError;
use crate::primary_key_part::PrimaryKeyPart;
use crate::primary_key_source::PrimaryKeySource;

pub(crate) struct PrimaryKeyColumns<'collected> {
    index: &'collected AttributeIndex,
    models: &'collected HashMap<CanonicalPath, &'collected CollectedModel>,
    resolved: HashMap<CanonicalPath, Vec<KeyColumn>>,
}

impl<'collected> PrimaryKeyColumns<'collected> {
    pub(crate) fn new(
        index: &'collected AttributeIndex,
        models: &'collected HashMap<CanonicalPath, &'collected CollectedModel>,
    ) -> Self {
        Self {
            index,
            models,
            resolved: HashMap::new(),
        }
    }

    pub(crate) fn reference(
        &mut self,
        field: &str,
        target: &CanonicalPath,
        model: &str,
    ) -> Result<KeyReference, ModelCodegenError> {
        let referenced = self.model(target, model, field)?;
        let target_columns = self.of(referenced, &mut HashSet::new())?;
        let mut columns: Vec<KeyColumn> = Vec::with_capacity(target_columns.len());
        let mut references_columns: Vec<String> = Vec::with_capacity(target_columns.len());

        for KeyColumn { column_type, name } in target_columns {
            columns.push(KeyColumn {
                column_type,
                name: schema_identifier(&[field, name.as_str()]).map_err(|source| {
                    ModelCodegenError::ForeignKeyColumnNameTooLong {
                        field: field.to_string(),
                        model: model.to_string(),
                        source,
                    }
                })?,
            });
            references_columns.push(name);
        }

        Ok(KeyReference {
            columns,
            references_columns,
            references_table: referenced.table.clone(),
        })
    }

    fn model(
        &self,
        target: &CanonicalPath,
        model: &str,
        field: &str,
    ) -> Result<&'collected CollectedModel, ModelCodegenError> {
        self.models
            .get(target)
            .copied()
            .ok_or_else(|| match self.index.item(target) {
                Some(_) => ModelCodegenError::KeyTargetNotAModel {
                    field: field.to_string(),
                    model: model.to_string(),
                    target: target.to_string(),
                },
                None => ModelCodegenError::KeyTargetOutsideCrate {
                    field: field.to_string(),
                    model: model.to_string(),
                    target: target.to_string(),
                },
            })
    }

    fn of(
        &mut self,
        collected: &'collected CollectedModel,
        visiting: &mut HashSet<CanonicalPath>,
    ) -> Result<Vec<KeyColumn>, ModelCodegenError> {
        if let Some(resolved) = self.resolved.get(&collected.path) {
            return Ok(resolved.clone());
        }

        if !visiting.insert(collected.path.clone()) {
            return Err(ModelCodegenError::PrimaryKeyReferenceCycle {
                model: collected.path.to_string(),
            });
        }

        let collected_path = collected.path.to_string();
        let mut columns: Vec<KeyColumn> = Vec::new();

        for PrimaryKeyPart {
            field: key_field,
            source,
        } in &collected.primary_key
        {
            match source {
                PrimaryKeySource::Column(column) => columns.push(column.clone()),
                PrimaryKeySource::Key { target } => {
                    let nested = self.model(target, &collected_path, key_field)?;

                    for KeyColumn { column_type, name } in self.of(nested, visiting)? {
                        columns.push(KeyColumn {
                            column_type,
                            name: schema_identifier(&[key_field.as_str(), name.as_str()]).map_err(
                                |source| ModelCodegenError::ForeignKeyColumnNameTooLong {
                                    field: key_field.clone(),
                                    model: collected_path.clone(),
                                    source,
                                },
                            )?,
                        });
                    }
                }
            }
        }

        visiting.remove(&collected.path);
        self.resolved
            .insert(collected.path.clone(), columns.clone());

        Ok(columns)
    }
}
