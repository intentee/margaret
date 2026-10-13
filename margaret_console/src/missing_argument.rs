use clap::error::Error;
use clap::error::ErrorKind;

use crate::command_outcome::CommandOutcome;
use crate::outcome_for_clap_error::outcome_for_clap_error;

pub(crate) fn missing_argument(id: &'static str) -> CommandOutcome {
    outcome_for_clap_error(&Error::raw(
        ErrorKind::MissingRequiredArgument,
        format!("the argument '{id}' is required\n"),
    ))
}
