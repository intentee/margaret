use clap::Command;

use crate::command_outcome::CommandOutcome;
use crate::console_output_error::ConsoleOutputError;
use crate::report_failure::report_failure;
use crate::write_standard_output::write_standard_output;

pub fn print_help(command: &mut Command) -> CommandOutcome {
    match write_standard_output(command.render_help()) {
        Ok(()) => CommandOutcome::Failed,
        Err(source) => report_failure(ConsoleOutputError::Help { source }),
    }
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use super::print_help;
    use crate::command_outcome::CommandOutcome;

    #[test]
    fn prints_help_and_reports_failure_since_no_subcommand_was_chosen() {
        let mut command = Command::new("app").subcommand(Command::new("greet"));

        assert_eq!(print_help(&mut command), CommandOutcome::Failed);
    }
}
