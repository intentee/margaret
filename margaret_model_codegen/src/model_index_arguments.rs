use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::column_list_arity::ColumnListArity;
use crate::explicit_index_name::explicit_index_name;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_column_list::ModelColumnList;

#[derive(Debug)]
pub(crate) struct ModelIndexArguments {
    pub(crate) columns: Vec<String>,
    pub(crate) name: String,
}

impl ModelIndexArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, model: &str) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?.ok_or_else(|| {
                ModelCodegenError::ModelIndexRequiresName {
                    model: model.to_string(),
                }
            })?;
            let ModelColumnList { columns } = ModelColumnList::read(
                reader,
                FrameworkAttribute::Index,
                ColumnListArity::TwoOrMore,
                model,
            )?;

            Ok(Self {
                columns,
                name: explicit_index_name(name, model)?,
            })
        })
    }
}
