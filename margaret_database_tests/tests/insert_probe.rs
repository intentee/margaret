use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::expression::Expression;
use margaret_sql::insert::Insert;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::insert_value::InsertValue;
use margaret_sql::render_insert::render_insert;
use margaret_sql::returning::Returning;
use margaret_sql::sql_parameter::SqlParameter;
use margaret_sql::statement::Statement;

use crate::probes::PROBES;

pub fn insert_probe(id: i64, amount: i64, conflict: ConflictAction) -> Statement {
    render_insert(&Insert {
        conflict,
        returning: Returning::Nothing,
        source: InsertSource::Values,
        target: PROBES,
        values: vec![
            InsertValue {
                column: "id",
                value: Expression::Parameter(SqlParameter::new(id)),
            },
            InsertValue {
                column: "amount",
                value: Expression::Parameter(SqlParameter::new(amount)),
            },
        ],
    })
}
