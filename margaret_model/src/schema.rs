use crate::table::Table;

#[derive(Debug, Eq, PartialEq)]
pub struct Schema {
    pub table_sets: &'static [&'static [&'static Table]],
}
