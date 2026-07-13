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
    fn reports_a_failed_outcome() {
        assert_eq!(report_failure("service registration failed"), CommandOutcome::Failed);
    }
}
