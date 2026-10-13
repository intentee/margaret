use margaret_model::check_predicate::CheckPredicate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCheck {
    pub name: String,
    pub predicate: CheckPredicate,
}
