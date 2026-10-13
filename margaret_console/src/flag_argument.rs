use clap::ArgMatches;

use crate::argument_lookup::argument_lookup;
use crate::command_outcome::CommandOutcome;
use crate::missing_argument::missing_argument;

/// # Errors
///
/// Returns the `CommandOutcome` the command ends with when the flag was not declared as a flag.
pub fn flag_argument(matches: &ArgMatches, id: &'static str) -> Result<bool, CommandOutcome> {
    argument_lookup::<bool>(matches, id)?.ok_or_else(|| missing_argument(id))
}

#[cfg(test)]
mod tests {
    use std::iter;

    use clap::Arg;
    use clap::ArgAction;
    use clap::ArgMatches;
    use clap::Command;

    use super::flag_argument;
    use crate::command_outcome::CommandOutcome;

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("app")
            .arg(Arg::new("loud").long("loud").action(ArgAction::SetTrue))
            .arg(Arg::new("name").long("name"))
            .try_get_matches_from(iter::once("app").chain(arguments.iter().copied()))
            .expect("the arguments parse")
    }

    #[test]
    fn reads_a_raised_flag_as_true() {
        assert_eq!(flag_argument(&matches(&["--loud"]), "loud"), Ok(true));
    }

    #[test]
    fn reads_an_unraised_flag_as_false() {
        assert_eq!(flag_argument(&matches(&[]), "loud"), Ok(false));
    }

    #[test]
    fn ends_the_command_as_failed_when_the_flag_has_no_value() {
        assert_eq!(
            flag_argument(&matches(&[]), "name"),
            Err(CommandOutcome::Failed)
        );
    }

    #[test]
    fn ends_the_command_as_failed_when_the_flag_holds_a_value_of_another_type() {
        assert_eq!(
            flag_argument(&matches(&["--name", "milo"]), "name"),
            Err(CommandOutcome::Failed)
        );
    }
}
