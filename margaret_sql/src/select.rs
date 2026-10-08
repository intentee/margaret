use crate::expression::Expression;
use crate::from_item::FromItem;
use crate::join::Join;
use crate::order_term::OrderTerm;
use crate::select_filter::SelectFilter;
use crate::select_limit::SelectLimit;

#[derive(Clone)]
pub struct Select {
    pub columns: Vec<Expression>,
    pub filter: SelectFilter,
    pub from: FromItem,
    pub joins: Vec<Join>,
    pub limit: SelectLimit,
    pub ordering: Vec<OrderTerm>,
}
