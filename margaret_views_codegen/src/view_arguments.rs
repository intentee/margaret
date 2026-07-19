use margaret_attributes::attribute_args::AttributeArgs;

use crate::views_codegen_error::ViewsCodegenError;

pub(crate) struct ViewArguments {
    pub(crate) name: String,
}

impl ViewArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        view: &str,
    ) -> Result<Self, ViewsCodegenError> {
        let name = arguments
            .string("name")?
            .ok_or_else(|| ViewsCodegenError::ViewMissingName {
                view: view.to_string(),
            })?;

        Ok(Self { name })
    }
}
