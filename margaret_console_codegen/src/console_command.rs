use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

pub(crate) struct ConsoleCommand {
    pub(crate) accessor: Ident,
    pub(crate) serve_input_slots: Vec<usize>,
    pub(crate) serve_inputs: Vec<ServeInput>,
    pub(crate) command_path: String,
    pub(crate) construction_root: CanonicalPath,
    pub(crate) description: Option<String>,
    pub(crate) is_async: bool,
    pub(crate) name: String,
    pub(crate) takes_token: bool,
}
