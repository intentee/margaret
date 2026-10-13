use margaret_sql::sql_parameter::SqlParameter;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;
use crate::record::Record;

pub(crate) struct PrimaryKeyParameters {
    values: Vec<SqlParameter>,
}

impl PrimaryKeyParameters {
    pub(crate) fn of<Keyed: Record>(record_parameters: &[SqlParameter]) -> Self {
        Self {
            values: Keyed::PRIMARY_KEY
                .iter()
                .flat_map(|span| {
                    record_parameters
                        .iter()
                        .skip(span.start)
                        .take(span.width)
                        .cloned()
                })
                .collect(),
        }
    }

    pub(crate) fn into_values(self) -> Vec<SqlParameter> {
        self.values
    }
}

impl Encodable for PrimaryKeyParameters {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        for value in &self.values {
            parameters.push_parameter(value.clone());
        }

        Ok(())
    }
}
