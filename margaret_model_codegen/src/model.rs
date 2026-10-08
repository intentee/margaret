use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;

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
    pub table: String,
    pub unique_constraints: Vec<ResolvedUniqueConstraint>,
}

impl Model {
    #[must_use]
    pub fn column_default(&self, column: &ResolvedColumn) -> ColumnDefault {
        match self.primary_key.as_slice() {
            [identity]
                if *identity == column.name && column.inferred.column_type == ColumnType::Uuid =>
            {
                ColumnDefault::UuidV7
            }
            _ => ColumnDefault::NotSet,
        }
    }
}
