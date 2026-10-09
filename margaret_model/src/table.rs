use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::column::Column;
use crate::foreign_key::ForeignKey;
use crate::index::Index;
use crate::unique_constraint::UniqueConstraint;

#[derive(Debug, Eq, PartialEq)]
pub struct Table {
    pub columns: &'static [Column],
    pub foreign_keys: &'static [ForeignKey],
    pub indexes: &'static [Index],
    pub name: &'static str,
    pub namespace: TableNamespace,
    pub primary_key: &'static [&'static str],
    pub unique_constraints: &'static [UniqueConstraint],
}
