use crate::table_source::TableSource;
use crate::unnest_source::UnnestSource;

#[derive(Clone)]
pub enum FromItem {
    Table(TableSource),
    Unnest(UnnestSource),
}
