use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::middleware_codegen_error::MiddlewareCodegenError;

pub(crate) struct MiddlewareAttributeArguments {
    pub(crate) handles: Path,
}

impl MiddlewareAttributeArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        middleware: &str,
    ) -> Result<Self, MiddlewareCodegenError> {
        arguments.expect_only(&["attribute"], &[])?;

        let handles = arguments.path("attribute")?.ok_or_else(|| {
            MiddlewareCodegenError::MissingMiddlewareHandles {
                middleware: middleware.to_string(),
            }
        })?;

        Ok(Self { handles })
    }
}
