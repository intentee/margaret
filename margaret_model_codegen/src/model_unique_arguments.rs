use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::column_list_arity::ColumnListArity;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_column_list::ModelColumnList;

#[derive(Debug)]
pub(crate) struct ModelUniqueArguments {
    pub(crate) columns: Vec<String>,
}

impl ModelUniqueArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, model: &str) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let ModelColumnList { columns } = ModelColumnList::read(
                reader,
                FrameworkAttribute::Unique,
                ColumnListArity::TwoOrMore,
                model,
            )?;

            Ok(Self { columns })
        })
    }
}
