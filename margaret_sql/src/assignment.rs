use crate::expression::Expression;

#[derive(Clone)]
pub struct Assignment {
    pub columns: Vec<&'static str>,
    pub values: Vec<Expression>,
}
