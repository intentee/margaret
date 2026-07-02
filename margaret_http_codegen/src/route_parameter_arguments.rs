use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct RouteParameterArguments {
    pub(crate) from: String,
}

impl RouteParameterArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        responder: &str,
        position: usize,
    ) -> Result<Self, HttpCodegenError> {
        let from = arguments.string("from")?.ok_or_else(|| {
            HttpCodegenError::RouteParameterMissingFrom {
                responder: responder.to_string(),
                parameter: position.to_string(),
            }
        })?;

        Ok(Self { from })
    }
}
