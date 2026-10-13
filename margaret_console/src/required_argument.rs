use std::any::Any;

use clap::ArgMatches;

use crate::argument_lookup::argument_lookup;
use crate::command_outcome::CommandOutcome;
use crate::missing_argument::missing_argument;

/// # Errors
///
/// Returns the `CommandOutcome` the command ends with when the argument is absent or was declared
/// with another type.
pub fn required_argument<Value>(
    matches: &ArgMatches,
    id: &'static str,
) -> Result<Value, CommandOutcome>
where
    Value: Any + Clone + Send + Sync + 'static,
{
    argument_lookup::<Value>(matches, id)?.ok_or_else(|| missing_argument(id))
}

#[cfg(test)]
mod tests {
    use std::iter;

    use clap::Arg;
    use clap::ArgMatches;
    use clap::Command;

    use super::required_argument;
    use crate::command_outcome::CommandOutcome;

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("app")
            .arg(Arg::new("name").long("name"))
            .try_get_matches_from(iter::once("app").chain(arguments.iter().copied()))
            .expect("the arguments parse")
    }

    #[test]
    fn reads_the_value_of_a_present_argument() {
        assert_eq!(
            required_argument::<String>(&matches(&["--name", "ada"]), "name"),
            Ok("ada".to_string())
        );
    }

    #[test]
    fn ends_the_command_as_failed_when_the_argument_is_absent() {
        assert_eq!(
            required_argument::<String>(&matches(&[]), "name"),
            Err(CommandOutcome::Failed)
        );
    }

    #[test]
    fn ends_the_command_as_failed_when_the_argument_was_declared_with_another_type() {
        assert_eq!(
            required_argument::<u16>(&matches(&["--name", "ada"]), "name"),
            Err(CommandOutcome::Failed)
        );
    }
}
