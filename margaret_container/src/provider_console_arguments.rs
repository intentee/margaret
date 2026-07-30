use std::sync::Arc;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;

#[derive(Debug)]
pub struct ProviderConsoleArguments {
    pub arguments: Arc<[ConsoleArgument]>,
    pub slots: Arc<[usize]>,
}
