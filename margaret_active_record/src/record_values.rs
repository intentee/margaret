use margaret_sql::expression::Expression;
use margaret_sql::insert_value::InsertValue;

use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::record::Record;

pub(crate) fn record_values<Written: Record>(
    record: &Written,
) -> Result<Vec<InsertValue>, ActiveRecordError> {
    let mut parameters = Parameters::new(Written::TABLE);

    record.write(&mut parameters).map(|()| {
        Written::TABLE
            .columns
            .iter()
            .zip(parameters.into_values())
            .map(|(column, parameter)| InsertValue {
                column: column.name,
                value: Expression::Parameter(parameter),
            })
            .collect()
    })
}
