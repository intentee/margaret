use proc_macro2::Ident;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;

pub(crate) struct ConsoleCommand {
    pub(crate) accessor: Ident,
    pub(crate) arguments: Vec<ConsoleArgument>,
    pub(crate) command_path: String,
    pub(crate) description: Option<String>,
    pub(crate) name: String,
    pub(crate) takes_token: bool,
}
