use crate::condition::Condition;

#[derive(Clone)]
pub enum ConflictFilter {
    Always,
    When(Condition),
}
