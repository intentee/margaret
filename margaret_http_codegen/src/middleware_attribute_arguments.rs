use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct MiddlewareAttributeArguments {
    pub(crate) handles: Path,
}

impl MiddlewareAttributeArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        middleware: &str,
    ) -> Result<Self, HttpCodegenError> {
        let handles = arguments.path("attribute")?.ok_or_else(|| {
            HttpCodegenError::MissingMiddlewareHandles {
                middleware: middleware.to_string(),
            }
        })?;

        Ok(Self { handles })
    }
}
