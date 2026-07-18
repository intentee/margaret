use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::service_kind::ServiceKind;

pub(crate) struct ServiceUnit {
    pub(crate) arguments: Vec<ConsoleArgument>,
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) field_name: String,
    pub(crate) kind: ServiceKind,
    pub(crate) runner: String,
    pub(crate) takes_token: bool,
    pub(crate) type_name: String,
}
