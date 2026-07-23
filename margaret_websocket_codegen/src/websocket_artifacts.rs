use std::collections::BTreeMap;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct WebSocketArtifacts {
    pub modules: Vec<GeneratedModuleTokens>,
    pub server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    pub servers: Vec<String>,
}
