use margaret_attributes::canonical_path::CanonicalPath;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::declared_relation::DeclaredRelation;
use crate::model_field::ModelField;
use crate::model_foreign_key::ModelForeignKey;
use crate::model_indexes::ModelIndexes;

pub(crate) struct AssembledModel {
    pub(crate) fields: Vec<ModelField>,
    pub(crate) foreign_keys: Vec<ModelForeignKey>,
    pub(crate) indexes: ModelIndexes,
    pub(crate) module: String,
    pub(crate) namespace: TableNamespace,
    pub(crate) path: CanonicalPath,
    pub(crate) relations: Vec<DeclaredRelation>,
    pub(crate) table: String,
}
