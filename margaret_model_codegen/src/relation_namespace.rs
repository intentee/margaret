use std::collections::HashMap;

use crate::index_kind::IndexKind;
use crate::model::Model;
use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct RelationNamespace {
    constraint_indexes: HashMap<String, String>,
    indexes: HashMap<String, String>,
    tables: HashMap<String, String>,
}

impl RelationNamespace {
    pub(crate) fn new() -> Self {
        Self {
            constraint_indexes: HashMap::new(),
            indexes: HashMap::new(),
            tables: HashMap::new(),
        }
    }

    pub(crate) fn register_indexes(&mut self, models: &[Model]) -> Result<(), ModelCodegenError> {
        for model in models {
            for index in model
                .indexes
                .all()
                .filter(|index| index.kind != IndexKind::Plain)
            {
                self.register_constraint_index(&index.name, &model.table)?;
            }
        }

        for model in models {
            for index in model.indexes.of_kind(IndexKind::Plain) {
                self.register_index(&index.name, &model.table)?;
            }
        }

        Ok(())
    }

    pub(crate) fn register_table(
        &mut self,
        table: &str,
        model: &str,
    ) -> Result<(), ModelCodegenError> {
        if let Some(first) = self.tables.get(table) {
            return Err(ModelCodegenError::DuplicateTableName {
                first: first.clone(),
                second: model.to_string(),
                table: table.to_string(),
            });
        }

        self.tables.insert(table.to_string(), model.to_string());

        Ok(())
    }

    fn register_constraint_index(
        &mut self,
        name: &str,
        table: &str,
    ) -> Result<(), ModelCodegenError> {
        if let Some(table_model) = self.tables.get(name) {
            return Err(ModelCodegenError::ConstraintIndexCollidesWithTableName {
                constraint_table: table.to_string(),
                name: name.to_string(),
                table_model: table_model.clone(),
            });
        }

        if let Some(first_table) = self.constraint_indexes.get(name) {
            return Err(ModelCodegenError::DuplicateConstraintIndexName {
                first_table: first_table.clone(),
                name: name.to_string(),
                second_table: table.to_string(),
            });
        }

        self.constraint_indexes
            .insert(name.to_string(), table.to_string());

        Ok(())
    }

    fn register_index(&mut self, name: &str, table: &str) -> Result<(), ModelCodegenError> {
        if let Some(table_model) = self.tables.get(name) {
            return Err(ModelCodegenError::IndexNameCollidesWithTableName {
                index_table: table.to_string(),
                name: name.to_string(),
                table_model: table_model.clone(),
            });
        }

        if let Some(constraint_table) = self.constraint_indexes.get(name) {
            return Err(ModelCodegenError::IndexNameCollidesWithConstraintIndex {
                constraint_table: constraint_table.clone(),
                index_table: table.to_string(),
                name: name.to_string(),
            });
        }

        if let Some(first) = self.indexes.get(name) {
            return Err(ModelCodegenError::DuplicateIndexName {
                first: first.clone(),
                name: name.to_string(),
                second: table.to_string(),
            });
        }

        self.indexes.insert(name.to_string(), table.to_string());

        Ok(())
    }
}
