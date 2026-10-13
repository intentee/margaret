use std::any::Any;

use clap::ArgMatches;

use crate::argument_lookup::argument_lookup;
use crate::command_outcome::CommandOutcome;

/// # Errors
///
/// Returns the `CommandOutcome` the command ends with when the argument was declared with another
/// type.
pub fn optional_argument<Value>(
    matches: &ArgMatches,
    id: &'static str,
) -> Result<Option<Value>, CommandOutcome>
where
    Value: Any + Clone + Send + Sync + 'static,
{
    argument_lookup::<Value>(matches, id)
}

#[cfg(test)]
mod tests {
    use std::iter;

    use clap::Arg;
    use clap::ArgMatches;
    use clap::Command;

    use super::optional_argument;

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("app")
            .arg(Arg::new("name").long("name"))
            .try_get_matches_from(iter::once("app").chain(arguments.iter().copied()))
            .expect("the arguments parse")
    }

    #[test]
    fn reads_an_absent_optional_argument_as_none() {
        assert_eq!(optional_argument::<String>(&matches(&[]), "name"), Ok(None));
    }

    #[test]
    fn reads_the_value_of_a_present_optional_argument() {
        assert_eq!(
            optional_argument::<String>(&matches(&["--name", "ada"]), "name"),
            Ok(Some("ada".to_string()))
        );
    }
}
