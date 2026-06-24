use clap::Error;

use crate::command_outcome::CommandOutcome;

pub fn outcome_for_clap_error(error: Error) -> CommandOutcome {
    error.print().expect("the clap error is rendered");

    if error.use_stderr() {
        CommandOutcome::Failed
    } else {
        CommandOutcome::Succeeded
    }
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use super::outcome_for_clap_error;
    use crate::command_outcome::CommandOutcome;

    fn clap_error(args: &[&str]) -> clap::Error {
        Command::new("app")
            .subcommand(Command::new("greet"))
            .try_get_matches_from(args)
            .expect_err("the arguments are rejected")
    }

    #[test]
    fn maps_a_usage_error_to_failure() {
        assert_eq!(
            outcome_for_clap_error(clap_error(&["app", "--unknown"])),
            CommandOutcome::Failed
        );
    }

    #[test]
    fn maps_help_to_success() {
        assert_eq!(
            outcome_for_clap_error(clap_error(&["app", "--help"])),
            CommandOutcome::Succeeded
        );
    }
}
