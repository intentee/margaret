use crate::table::Table;

pub struct Schema {
    pub framework_tables: Vec<Table>,
    pub tables: Vec<Table>,
}
