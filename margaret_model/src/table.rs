use crate::column::Column;
use crate::foreign_key::ForeignKey;

pub struct Table {
    pub columns: Vec<Column>,
    pub foreign_keys: Vec<ForeignKey>,
    pub name: String,
    pub primary_key: Vec<String>,
}
