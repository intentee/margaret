use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct CanArguments {
    pub(crate) action: Path,
}

impl CanArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        responder: &str,
    ) -> Result<Self, HttpCodegenError> {
        let action = arguments
            .positional_path(0)
            .ok_or_else(|| HttpCodegenError::CanWithoutAction {
                responder: responder.to_string(),
            })?
            .clone();

        Ok(Self { action })
    }
}
