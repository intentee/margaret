use crate::condition::Condition;

#[derive(Clone)]
pub enum SelectFilter {
    Everything,
    Matching(Condition),
}
