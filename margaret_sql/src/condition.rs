use crate::comparison::Comparison;
use crate::exists_probe::ExistsProbe;
use crate::expression::Expression;

#[derive(Clone)]
pub enum Condition {
    And {
        left: Box<Condition>,
        right: Box<Condition>,
    },
    Compare {
        comparison: Comparison,
        left: Expression,
        right: Expression,
    },
    Exists(ExistsProbe),
    IsNull(Vec<Expression>),
    Not(Box<Condition>),
    RowCompare {
        comparison: Comparison,
        left: Vec<Expression>,
        right: Vec<Expression>,
    },
}
