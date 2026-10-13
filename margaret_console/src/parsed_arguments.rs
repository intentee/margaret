use std::ffi::OsString;

use clap::ArgMatches;
use clap::Command;

use crate::command_outcome::CommandOutcome;
use crate::outcome_for_clap_error::outcome_for_clap_error;

/// # Errors
///
/// Returns the `CommandOutcome` the command ends with when clap rejects the arguments or answers
/// them with help or version text.
pub fn parsed_arguments<Arguments, Argument>(
    command: &mut Command,
    args: Arguments,
) -> Result<ArgMatches, CommandOutcome>
where
    Arguments: IntoIterator<Item = Argument>,
    Argument: Into<OsString> + Clone,
{
    command
        .try_get_matches_from_mut(args)
        .map_err(|error| outcome_for_clap_error(&error))
}

#[cfg(test)]
mod tests {
    use clap::Arg;
    use clap::Command;

    use super::parsed_arguments;
    use crate::command_outcome::CommandOutcome;

    fn command() -> Command {
        Command::new("app").arg(Arg::new("name").long("name"))
    }

    #[test]
    fn parses_arguments_clap_accepts() {
        let matches = parsed_arguments(&mut command(), ["app", "--name", "ada"])
            .expect("the arguments parse");

        assert_eq!(
            matches.get_one::<String>("name").map(String::as_str),
            Some("ada")
        );
    }

    #[test]
    fn ends_the_command_with_the_outcome_of_the_clap_error() {
        assert_eq!(
            parsed_arguments(&mut command(), ["app", "--unknown"]).err(),
            Some(CommandOutcome::Failed)
        );
    }
}
