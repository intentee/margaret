use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_commands::console_commands;
use crate::render::render;

pub fn render_console(
    index: &AttributeIndex,
    serves: bool,
    has_models: bool,
    servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
) -> Result<GeneratedModuleTokens, ConsoleCodegenError> {
    let commands = console_commands(index)?;

    Ok(GeneratedModuleTokens::new(
        "run",
        render(&commands, serves, has_models, servers, serve_arguments),
    ))
}
