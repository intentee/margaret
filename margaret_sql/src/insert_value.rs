use crate::expression::Expression;

#[derive(Clone)]
pub struct InsertValue {
    pub column: &'static str,
    pub value: Expression,
}
