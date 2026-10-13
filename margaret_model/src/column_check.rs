use crate::check_predicate::CheckPredicate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColumnCheck {
    pub name: &'static str,
    pub predicate: CheckPredicate,
}
