use crate::assignments::Assignments;
use crate::conflict_filter::ConflictFilter;

#[derive(Clone)]
pub enum ConflictAction {
    Ignore {
        target: Vec<&'static str>,
    },
    Raise,
    Update {
        assignments: Assignments,
        filter: ConflictFilter,
        target: Vec<&'static str>,
    },
}
