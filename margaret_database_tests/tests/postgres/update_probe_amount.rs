use margaret_sql::assignment::Assignment;
use margaret_sql::assignments::Assignments;
use margaret_sql::comparison::Comparison;
use margaret_sql::condition::Condition;
use margaret_sql::expression::Expression;
use margaret_sql::render_update::render_update;
use margaret_sql::returning::Returning;
use margaret_sql::sql_parameter::SqlParameter;
use margaret_sql::statement::Statement;
use margaret_sql::update::Update;

use crate::postgres::probes::PROBES;

pub fn update_probe_amount(id: i64, amount: i64) -> Statement {
    render_update(&Update {
        assignments: Assignments {
            first: Assignment {
                columns: vec!["amount"],
                values: vec![Expression::Parameter(SqlParameter::new(amount))],
            },
            rest: Vec::new(),
        },
        condition: Condition::Compare {
            comparison: Comparison::Equal,
            left: Expression::Column {
                alias: PROBES.alias,
                column: "id",
            },
            right: Expression::Parameter(SqlParameter::new(id)),
        },
        returning: Returning::Nothing,
        target: PROBES,
    })
}
