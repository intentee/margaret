use std::collections::BTreeMap;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;

pub struct ServeConsoleArguments<'arguments> {
    pub serve_arguments: &'arguments [ConsoleArgument],
    pub server_console_arguments: &'arguments BTreeMap<String, Vec<ConsoleArgument>>,
    pub views_console_arguments: &'arguments [ConsoleArgument],
}
