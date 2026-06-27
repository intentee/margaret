use proc_macro2::Ident;

use crate::console_argument::ConsoleArgument;

pub(crate) struct ConsoleCommand {
    pub(crate) accessor: Ident,
    pub(crate) arguments: Vec<ConsoleArgument>,
    pub(crate) description: Option<String>,
    pub(crate) name: String,
    pub(crate) takes_token: bool,
}
