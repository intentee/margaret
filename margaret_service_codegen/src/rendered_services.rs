use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

pub struct RenderedServices {
    pub module: GeneratedModuleTokens,
    pub serve_arguments: Vec<ConsoleArgument>,
}
