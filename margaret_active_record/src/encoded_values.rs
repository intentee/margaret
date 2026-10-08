use margaret_model::table::Table;
use margaret_sql::expression::Expression;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;

pub(crate) fn encoded_values(
    value: &dyn Encodable,
    table: &'static Table,
) -> Result<Vec<Expression>, ActiveRecordError> {
    let mut parameters = Parameters::new(table);

    value.encode(&mut parameters).map(|()| {
        parameters
            .into_values()
            .into_iter()
            .map(Expression::Parameter)
            .collect()
    })
}
