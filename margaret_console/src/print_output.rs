use std::fmt::Display;

use crate::command_outcome::CommandOutcome;
use crate::console_output_error::ConsoleOutputError;
use crate::report_failure::report_failure;
use crate::write_standard_output::write_standard_output;

pub fn print_output(output: impl Display) -> CommandOutcome {
    match write_standard_output(output) {
        Ok(()) => CommandOutcome::Succeeded,
        Err(source) => report_failure(ConsoleOutputError::CommandOutput { source }),
    }
}

#[cfg(test)]
mod tests {
    use super::print_output;
    use crate::command_outcome::CommandOutcome;

    #[test]
    fn prints_the_output_of_a_command_that_succeeded() {
        assert_eq!(print_output("the schema"), CommandOutcome::Succeeded);
    }
}
