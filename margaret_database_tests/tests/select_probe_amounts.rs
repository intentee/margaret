use margaret_sql::direction::Direction;
use margaret_sql::expression::Expression;
use margaret_sql::from_item::FromItem;
use margaret_sql::order_term::OrderTerm;
use margaret_sql::render_select::render_select;
use margaret_sql::select::Select;
use margaret_sql::select_filter::SelectFilter;
use margaret_sql::select_limit::SelectLimit;
use margaret_sql::statement::Statement;

use crate::probes::PROBES;

pub fn select_probe_amounts() -> Statement {
    render_select(&Select {
        columns: vec![Expression::Column {
            alias: PROBES.alias,
            column: "amount",
        }],
        filter: SelectFilter::Everything,
        from: FromItem::Table(PROBES),
        joins: Vec::new(),
        limit: SelectLimit::Unlimited,
        ordering: vec![OrderTerm {
            direction: Direction::Ascending,
            expression: Expression::Column {
                alias: PROBES.alias,
                column: "id",
            },
        }],
    })
}
