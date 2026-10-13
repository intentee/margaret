use margaret_sql::sql_parameter::SqlParameter;

use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::record::Record;

pub(crate) fn record_parameters<Encoded: Record>(
    record: &Encoded,
) -> Result<Vec<SqlParameter>, ActiveRecordError> {
    let mut parameters = Parameters::new(Encoded::TABLE);

    record
        .write(&mut parameters)
        .map(|()| parameters.into_values())
}
