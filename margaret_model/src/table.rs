use crate::column::Column;

pub struct Table {
    pub columns: Vec<Column>,
    pub name: String,
    pub primary_key: Vec<String>,
}
