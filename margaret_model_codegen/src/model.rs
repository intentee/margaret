use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;

#[derive(Debug)]
pub struct Model {
    pub columns: Vec<ResolvedColumn>,
    pub foreign_keys: Vec<ResolvedForeignKey>,
    pub indexes: Vec<ResolvedIndex>,
    pub primary_key: Vec<String>,
    pub table: SchemaIdentifier,
    pub unique_constraints: Vec<ResolvedUniqueConstraint>,
}
