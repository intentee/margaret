use crate::expression::Expression;

#[derive(Clone)]
pub enum Returning {
    Columns(Vec<Expression>),
    Nothing,
}
