use margaret_attributes::attribute_index::AttributeIndex;
use margaret_http_codegen::http_server::HttpServer;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_commands::console_commands;
use crate::render::render;

pub fn render_console(
    index: &AttributeIndex,
    serves: bool,
    servers: &[HttpServer],
) -> Result<String, ConsoleCodegenError> {
    let commands = console_commands(index)?;

    Ok(render(&commands, serves, servers))
}
