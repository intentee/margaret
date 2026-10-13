use crate::conflict_action::ConflictAction;
use crate::insert_source::InsertSource;
use crate::insert_value::InsertValue;
use crate::returning::Returning;
use crate::table_source::TableSource;

#[derive(Clone)]
pub struct Insert {
    pub conflict: ConflictAction,
    pub returning: Returning,
    pub source: InsertSource,
    pub target: TableSource,
    pub values: Vec<InsertValue>,
}
