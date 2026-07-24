use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

pub(crate) enum DependencyKind {
    ConsoleArgument { argument: Box<ConsoleArgument> },
    Single { provider_key: CanonicalPath },
}
