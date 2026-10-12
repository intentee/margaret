use std::fmt::Display;

use crate::command_outcome::CommandOutcome;

pub fn report_failure(error: impl Display) -> CommandOutcome {
    eprintln!("{error}");

    CommandOutcome::Failed
}

#[cfg(test)]
mod tests {
    use crate::command_outcome::CommandOutcome;

    use super::report_failure;

    #[test]
    fn reports_an_error_as_a_failed_outcome_so_the_command_exits_with_failure() {
        assert_eq!(
            report_failure("service registration failed"),
            CommandOutcome::Failed
        );
    }
}
