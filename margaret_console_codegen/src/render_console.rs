use margaret_attributes::attribute_index::AttributeIndex;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_commands::console_commands;
use crate::render::render;

pub fn render_console(
    index: &AttributeIndex,
    serves: bool,
    has_http: bool,
) -> Result<String, ConsoleCodegenError> {
    let commands = console_commands(index)?;

    Ok(render(&commands, serves, has_http))
}
