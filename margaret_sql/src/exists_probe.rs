use crate::condition::Condition;
use crate::table_source::TableSource;

#[derive(Clone)]
pub struct ExistsProbe {
    pub condition: Box<Condition>,
    pub source: TableSource,
}
