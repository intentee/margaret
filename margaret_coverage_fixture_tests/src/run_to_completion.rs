use std::ffi::OsString;

use tokio::runtime::Builder;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

/// # Panics
///
/// Panics when the runtime the command runs on cannot be built.
pub fn run_to_completion<Arguments, Argument>(arguments: Arguments) -> CommandOutcome
where
    Arguments: IntoIterator<Item = Argument>,
    Argument: Into<OsString> + Clone,
{
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("the command runtime builds")
        .block_on(run(arguments))
}
