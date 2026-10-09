use margaret_attributes::canonical_path::CanonicalPath;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::model_field::ModelField;
use crate::model_foreign_key::ModelForeignKey;
use crate::model_indexes::ModelIndexes;
use crate::model_relation::ModelRelation;
use crate::resolved_column::ResolvedColumn;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    pub fields: Vec<ModelField>,
    pub foreign_keys: Vec<ModelForeignKey>,
    pub indexes: ModelIndexes,
    pub module: String,
    pub namespace: TableNamespace,
    pub path: CanonicalPath,
    pub relations: Vec<ModelRelation>,
    pub table: String,
}

impl Model {
    pub fn columns(&self) -> impl Iterator<Item = &ResolvedColumn> {
        self.fields.iter().flat_map(|field| field.columns.iter())
    }
}
