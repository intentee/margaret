use std::collections::HashMap;

use crate::column_id::ColumnId;
use crate::foreign_key_target_column::ForeignKeyTargetColumn;

pub(crate) struct ForeignKeyTarget {
    pub(crate) columns_by_field: HashMap<String, ForeignKeyTargetColumn>,
    pub(crate) primary_key: Vec<ColumnId>,
    pub(crate) table: String,
    pub(crate) unique_keys: Vec<Vec<ColumnId>>,
}
