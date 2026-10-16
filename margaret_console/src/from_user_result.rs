use crate::command_outcome::CommandOutcome;
use crate::report_failure::report_failure;

impl CommandOutcome {
    #[must_use]
    pub fn from_user_result(result: anyhow::Result<Self>) -> Self {
        match result {
            Ok(outcome) => outcome,
            Err(error) => report_failure(format_args!("{error:#}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_outcome::CommandOutcome;

    #[test]
    fn preserves_a_successful_user_result_so_the_process_exits_successfully() {
        assert_eq!(
            CommandOutcome::from_user_result(Ok(CommandOutcome::Succeeded)),
            CommandOutcome::Succeeded
        );
    }

    #[test]
    fn turns_a_failed_user_result_into_a_failed_outcome() {
        assert_eq!(
            CommandOutcome::from_user_result(Err(anyhow::anyhow!("database unavailable"))),
            CommandOutcome::Failed
        );
    }
}
