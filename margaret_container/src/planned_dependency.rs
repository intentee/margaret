use margaret_console_argument_codegen::console_argument::ConsoleArgument;

pub(crate) enum PlannedDependency {
    ConsoleArgument {
        argument: ConsoleArgument,
        slot: usize,
    },
    Single {
        field_name: String,
    },
}
