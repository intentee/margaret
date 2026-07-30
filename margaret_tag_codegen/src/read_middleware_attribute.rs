use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_error::AttributeError;

/// # Errors
///
/// Returns `AttributeError` propagated from the work it performs.
pub fn read_middleware_attribute(args: &AttributeArgs) -> Result<Option<Path>, AttributeError> {
    args.interpret(|reader| reader.take_path("attribute"))
}
