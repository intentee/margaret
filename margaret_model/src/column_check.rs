use crate::check_predicate::CheckPredicate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnCheck {
    pub name: String,
    pub predicate: CheckPredicate,
}
