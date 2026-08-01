use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;

/// # Errors
///
/// Returns `AttributeArgumentsError` propagated from the work it performs.
pub fn read_middleware_attribute(
    args: &AttributeArgs,
) -> Result<Option<Path>, AttributeArgumentsError> {
    args.interpret(|reader| reader.take_path("attribute"))
}
