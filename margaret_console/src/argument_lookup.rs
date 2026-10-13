use std::any::Any;

use clap::ArgMatches;

use crate::command_outcome::CommandOutcome;
use crate::report_failure::report_failure;

pub(crate) fn argument_lookup<Value>(
    matches: &ArgMatches,
    id: &'static str,
) -> Result<Option<Value>, CommandOutcome>
where
    Value: Any + Clone + Send + Sync + 'static,
{
    matches
        .try_get_one::<Value>(id)
        .map(Option::<&Value>::cloned)
        .map_err(report_failure)
}
