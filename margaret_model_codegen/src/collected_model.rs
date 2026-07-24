use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

use crate::column_id::ColumnId;
use crate::deferred_foreign_key_member::DeferredForeignKeyMember;
use crate::resolved_column::ResolvedColumn;

pub(crate) struct CollectedModel {
    pub(crate) columns: Vec<ResolvedColumn>,
    pub(crate) deferred_foreign_key_members: Vec<DeferredForeignKeyMember>,
    pub(crate) model: String,
    pub(crate) primary_key: Vec<ColumnId>,
    pub(crate) table: SchemaIdentifier,
    pub(crate) unique_keys: Vec<Vec<ColumnId>>,
}
