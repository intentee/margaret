use margaret_model::column_default::ColumnDefault;
use margaret_model::table::Table;
use margaret_sql::expression::Expression;
use margaret_sql::insert_value::InsertValue;

use crate::active_record_error::ActiveRecordError;
use crate::draft_record::DraftRecord;
use crate::parameters::Parameters;

pub(crate) fn draft_values(
    draft: &impl DraftRecord,
    table: &'static Table,
) -> Result<Vec<InsertValue>, ActiveRecordError> {
    let mut parameters = Parameters::new(table);

    draft.write(&mut parameters).map(|()| {
        table
            .columns
            .iter()
            .filter(|column| column.default == ColumnDefault::NotSet)
            .zip(parameters.into_values())
            .map(|(column, parameter)| InsertValue {
                column: column.name,
                value: Expression::Parameter(parameter),
            })
            .collect()
    })
}
