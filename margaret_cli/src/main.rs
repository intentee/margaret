use margaret_console::command_outcome::CommandOutcome;

fn main() -> CommandOutcome {
    margaret_scaffold::run::run(std::env::args_os())
}
