use crate::direction::Direction;
use crate::expression::Expression;

#[derive(Clone)]
pub struct OrderTerm {
    pub direction: Direction,
    pub expression: Expression,
}
