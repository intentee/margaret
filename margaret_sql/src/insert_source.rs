use crate::condition::Condition;

#[derive(Clone)]
pub enum InsertSource {
    Guarded(Condition),
    Values,
}
