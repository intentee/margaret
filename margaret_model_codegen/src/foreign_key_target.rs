use crate::foreign_key_target_column::ForeignKeyTargetColumn;

pub(crate) struct ForeignKeyTarget {
    pub(crate) primary_key: Vec<ForeignKeyTargetColumn>,
    pub(crate) table: String,
}
