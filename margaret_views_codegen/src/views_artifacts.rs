use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[derive(Debug)]
pub struct ViewsArtifacts {
    pub console_arguments: Vec<ConsoleArgument>,
    pub modules: Vec<GeneratedModuleTokens>,
}
