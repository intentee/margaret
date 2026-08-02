use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::views_codegen_error::ViewsCodegenError;

pub(crate) struct ViewArguments {
    pub(crate) name: String,
}

impl ViewArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, view: &str) -> Result<Self, ViewsCodegenError> {
        arguments.interpret(|reader| {
            let name =
                reader
                    .take_string("name")?
                    .ok_or_else(|| ViewsCodegenError::ViewMissingName {
                        view: view.to_string(),
                    })?;

            Ok(Self { name })
        })
    }
}
