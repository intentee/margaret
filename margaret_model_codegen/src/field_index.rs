use margaret_schema_identifier_naming::index_name::index_name;

use crate::model_codegen_error::ModelCodegenError;
use crate::resolved_index::ResolvedIndex;

#[derive(Debug)]
pub(crate) enum FieldIndex {
    Absent,
    Derived,
    Explicit(String),
}

impl FieldIndex {
    pub(crate) fn extend(
        self,
        indexes: &mut Vec<ResolvedIndex>,
        columns: Vec<String>,
        table: &str,
        model: &str,
    ) -> Result<(), ModelCodegenError> {
        match self {
            FieldIndex::Absent => Ok(()),
            FieldIndex::Derived => {
                let name = index_name(table, &columns).map_err(|source| {
                    ModelCodegenError::IndexNameTooLong {
                        model: model.to_string(),
                        source,
                    }
                })?;

                indexes.push(ResolvedIndex { columns, name });

                Ok(())
            }
            FieldIndex::Explicit(name) => {
                indexes.push(ResolvedIndex { columns, name });

                Ok(())
            }
        }
    }
}
