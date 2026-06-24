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
    use std::process::Termination;

    use super::CommandOutcome;

    #[test]
    fn reports_an_exit_code_for_each_outcome() {
        let _ = CommandOutcome::Succeeded.report();
        let _ = CommandOutcome::Failed.report();
    }
}
