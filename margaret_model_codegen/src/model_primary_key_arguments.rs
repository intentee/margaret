use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::model_codegen_error::ModelCodegenError;
use crate::model_field_list::ModelFieldList;

#[derive(Debug)]
pub(crate) struct ModelPrimaryKeyArguments {
    pub(crate) fields: Vec<String>,
}

impl ModelPrimaryKeyArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, model: &str) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let ModelFieldList { fields } =
                ModelFieldList::read(reader, FrameworkAttribute::PrimaryKey, model)?;

            Ok(Self { fields })
        })
    }
}
