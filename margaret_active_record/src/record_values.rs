use margaret_sql::expression::Expression;
use margaret_sql::insert_value::InsertValue;

use crate::active_record_error::ActiveRecordError;
use crate::record::Record;
use crate::record_parameters::record_parameters;

pub(crate) fn record_values<Written: Record>(
    record: &Written,
) -> Result<Vec<InsertValue>, ActiveRecordError> {
    record_parameters(record).map(|parameters| {
        Written::TABLE
            .columns
            .iter()
            .zip(parameters)
            .map(|(column, parameter)| InsertValue {
                column: column.name,
                value: Expression::Parameter(parameter),
            })
            .collect()
    })
}
