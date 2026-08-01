use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_tag_codegen::read_middleware_attribute::read_middleware_attribute;

use crate::middleware_codegen_error::MiddlewareCodegenError;

pub(crate) struct MiddlewareAttributeArguments {
    pub(crate) handles: Path,
}

impl MiddlewareAttributeArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        middleware: &str,
    ) -> Result<Self, MiddlewareCodegenError> {
        let handles = read_middleware_attribute(arguments)?.ok_or_else(|| {
            MiddlewareCodegenError::MissingMiddlewareHandles {
                middleware: middleware.to_string(),
            }
        })?;

        Ok(Self { handles })
    }
}
