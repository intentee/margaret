use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;

use crate::active_record_codegen_error::ActiveRecordCodegenError;

pub(crate) struct RelationArguments {
    pub(crate) limit: Option<usize>,
    pub(crate) relation: String,
}

impl RelationArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        shape: &str,
        field: &str,
    ) -> Result<Self, ActiveRecordCodegenError> {
        arguments.interpret(|reader| {
            let named = reader.take_positional_path().ok_or_else(|| {
                ActiveRecordCodegenError::RelationRequiresName {
                    field: field.to_string(),
                    shape: shape.to_string(),
                }
            })?;
            let relation = named.get_ident().map(ToString::to_string).ok_or_else(|| {
                ActiveRecordCodegenError::RelationNameIsNotAnIdentifier {
                    field: field.to_string(),
                    relation: format_path(&named),
                    shape: shape.to_string(),
                }
            })?;

            Ok(Self {
                limit: reader.take_unsigned_integer("limit")?,
                relation,
            })
        })
    }
}
