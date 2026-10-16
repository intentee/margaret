use std::env;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

#[tokio::main(flavor = "current_thread")]
async fn main() -> CommandOutcome {
    run(env::args_os()).await
}
