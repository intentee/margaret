use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

pub enum FrameworkDependency {
    ConsoleArgument(Box<ConsoleArgument>),
    Endpoint(CanonicalPath),
    Provider(CanonicalPath),
}
