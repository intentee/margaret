use clap::Error;

use crate::command_outcome::CommandOutcome;

#[must_use]
pub(crate) fn outcome_for_clap_error(error: &Error) -> CommandOutcome {
    if error.use_stderr() {
        eprintln!("{}", error.render());

        CommandOutcome::Failed
    } else {
        println!("{}", error.render());

        CommandOutcome::Succeeded
    }
}

#[cfg(test)]
mod tests {
    use clap::Command;
    use clap::Error;

    use super::outcome_for_clap_error;
    use crate::command_outcome::CommandOutcome;

    fn clap_error(args: &[&str]) -> Error {
        Command::new("app")
            .subcommand(Command::new("greet"))
            .try_get_matches_from(args)
            .expect_err("the arguments are rejected")
    }

    #[test]
    fn maps_a_usage_error_to_failure_so_scripts_see_a_nonzero_exit_status() {
        assert_eq!(
            outcome_for_clap_error(&clap_error(&["app", "--unknown"])),
            CommandOutcome::Failed
        );
    }

    #[test]
    fn maps_a_help_request_to_success_because_printing_help_is_not_an_error() {
        assert_eq!(
            outcome_for_clap_error(&clap_error(&["app", "--help"])),
            CommandOutcome::Succeeded
        );
    }
}
