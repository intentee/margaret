use std::process::ExitCode;
use std::process::Termination;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandOutcome {
    Succeeded,
    Failed,
}

impl Termination for CommandOutcome {
    fn report(self) -> ExitCode {
        match self {
            CommandOutcome::Succeeded => ExitCode::SUCCESS,
            CommandOutcome::Failed => ExitCode::FAILURE,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::ExitCode;
    use std::process::Termination;

    use super::CommandOutcome;

    #[test]
    fn reports_success_exit_code_for_a_succeeded_outcome() {
        assert_eq!(CommandOutcome::Succeeded.report(), ExitCode::SUCCESS);
    }

    #[test]
    fn reports_failure_exit_code_for_a_failed_outcome() {
        assert_eq!(CommandOutcome::Failed.report(), ExitCode::FAILURE);
    }
}
