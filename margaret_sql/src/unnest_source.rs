use crate::array_parameter::ArrayParameter;
use crate::table_alias::TableAlias;

#[derive(Clone)]
pub struct UnnestSource {
    pub alias: TableAlias,
    pub arrays: Vec<ArrayParameter>,
}
