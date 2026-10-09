use crate::condition::Condition;
use crate::returning::Returning;
use crate::table_source::TableSource;

#[derive(Clone)]
pub struct Delete {
    pub condition: Condition,
    pub returning: Returning,
    pub target: TableSource,
}
