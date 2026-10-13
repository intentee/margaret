use crate::key_column::KeyColumn;

pub(crate) struct KeyReference {
    pub(crate) columns: Vec<KeyColumn>,
    pub(crate) references_columns: Vec<String>,
    pub(crate) references_table: String,
}
