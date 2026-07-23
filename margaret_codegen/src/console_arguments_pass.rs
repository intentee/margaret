use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument_registry::ConsoleArgumentRegistry;
use margaret_console_argument_codegen::scan::scan;

use crate::codegen_error::CodegenError;

pub(crate) fn console_arguments_pass(
    index: &AttributeIndex,
) -> Result<ConsoleArgumentRegistry, CodegenError> {
    Ok(scan(index)?)
}
