use margaret_model::table::Table;
use margaret_sql::assignment::Assignment;
use margaret_sql::expression::Expression;

use crate::active_record_error::ActiveRecordError;
use crate::assigned_value::AssignedValue;
use crate::base_alias::BASE_ALIAS;
use crate::encoded_values::encoded_values;

pub struct PendingAssignment {
    pub(crate) columns: Vec<&'static str>,
    pub(crate) value: AssignedValue,
}

impl PendingAssignment {
    pub(crate) fn assignment(
        &self,
        table: &'static Table,
    ) -> Result<Assignment, ActiveRecordError> {
        match &self.value {
            AssignedValue::Excluded => Ok(self
                .columns
                .iter()
                .map(|column| Expression::Excluded { column })
                .collect()),
            AssignedValue::Given(value) => encoded_values(value.as_ref(), table),
            AssignedValue::Greatest => Ok(self
                .columns
                .iter()
                .map(|column| Expression::Greatest {
                    first: Box::new(Expression::Column {
                        alias: BASE_ALIAS,
                        column,
                    }),
                    second: Box::new(Expression::Excluded { column }),
                })
                .collect()),
        }
        .map(|values| Assignment {
            columns: self.columns.clone(),
            values,
        })
    }
}
